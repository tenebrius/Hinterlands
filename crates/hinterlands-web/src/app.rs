//! The app: owns the `SimHandle`, the `ViewState`, the layers and the panels,
//! and routes every widget's messages through `view::update`.

use crate::camera::Camera;
use crate::layers::{self, DrawCtx, Layer};
use crate::panels::{self, Panel};
use crate::sim_handle::SimHandle;
use crate::view::{self, Msg, ViewState};
use hinterlands_core::Params;
use serde::{Deserialize, Serialize};

/// A save is the run's identity: replaying it reproduces the state exactly.
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub map_seed: u64,
    pub sim_seed: u64,
    pub year: u32,
    pub params: Params,
}

impl SaveFile {
    pub fn of(v: &ViewState, sim: &SimHandle) -> Self {
        Self {
            map_seed: sim.map_seed,
            sim_seed: sim.sim_seed(),
            year: v.snapshot.year,
            params: sim.params().clone(),
        }
    }
}

/// Offer a text file to the user. Returns a status line.
pub fn download(name: &str, text: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let run = || -> Option<()> {
            let parts = js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(text));
            let opts = web_sys::BlobPropertyBag::new();
            opts.set_type("application/json");
            let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &opts).ok()?;
            let url = web_sys::Url::create_object_url_with_blob(&blob).ok()?;
            let doc = web_sys::window()?.document()?;
            let a = doc
                .create_element("a")
                .ok()?
                .dyn_into::<web_sys::HtmlAnchorElement>()
                .ok()?;
            a.set_href(&url);
            a.set_download(name);
            a.click();
            web_sys::Url::revoke_object_url(&url).ok()
        };
        if run().is_some() {
            format!("Saved {name}")
        } else {
            "Save failed".to_string()
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        match std::fs::write(name, text) {
            Ok(()) => format!("Saved {name}"),
            Err(e) => format!("Save failed: {e}"),
        }
    }
}

pub struct App {
    sim: SimHandle,
    view: ViewState,
    layers: Vec<Box<dyn Layer>>,
    panels: Vec<Box<dyn Panel>>,
    /// Fractional years owed to the clock.
    owed: f64,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let sim = SimHandle::new(1, 1, Params::default());
        let layers = layers::all();
        let on = layers
            .iter()
            .filter(|l| l.default_on())
            .map(|l| l.id())
            .collect();
        let mut view = ViewState::new(&sim, on);
        let mut sim = sim;
        if let Some(map) = url_param("map") {
            sim = SimHandle::new(
                map as u64,
                url_param("seed").unwrap_or(1) as u64,
                Params::default(),
            );
            view = ViewState::new(&sim, view.layers_on.clone());
        }
        // `?year=N` opens the run already advanced to year N.
        if let Some(year) = url_param("year") {
            view::update(&mut view, &mut sim, Msg::Step(year));
        }
        if let Some(years) = selftest_years() {
            // Cross-platform determinism check: compare with `hinterlands hash --years N`.
            let mut probe = SimHandle::new(1, 1, Params::default());
            let start = web_time_now();
            probe.step(years);
            let ms = (web_time_now() - start) / years.max(1) as f64;
            let line = format!(
                "selftest map 1 seed 1 years {years} hash {:016x} {ms:.1} ms/year",
                probe.snapshot().hash()
            );
            log::warn!("{line}");
            view.status = line;
        }
        Self {
            sim,
            view,
            layers,
            panels: panels::all(),
            owed: 0.0,
        }
    }

    fn apply(&mut self, msgs: Vec<Msg>) {
        for m in msgs {
            view::update(&mut self.view, &mut self.sim, m);
        }
    }

    fn map_ui(&mut self, ui: &mut egui::Ui) -> Vec<Msg> {
        let mut out = Vec::new();
        let (rect, resp) =
            ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let g = self.view.terrain.grid;
        let mut cam = self
            .view
            .camera
            .unwrap_or_else(|| Camera::fit(g.w, g.h, rect));
        let mut cam_changed = self.view.camera.is_none();
        if resp.dragged() {
            cam.center -= resp.drag_delta() / cam.zoom;
            cam_changed = true;
        }
        if let Some(pos) = resp.hover_pos() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                cam.zoom_at(rect, pos, (scroll * 0.002).exp());
                cam_changed = true;
            }
            let cell = cam.pick(rect, g.w, g.h, pos);
            if cell != self.view.hovered {
                out.push(Msg::Hover(cell));
            }
        } else if self.view.hovered.is_some() {
            out.push(Msg::Hover(None));
        }
        if resp.clicked() {
            out.push(Msg::Select(
                resp.interact_pointer_pos()
                    .and_then(|p| cam.pick(rect, g.w, g.h, p)),
            ));
        }
        if cam_changed {
            out.push(Msg::SetCamera(cam));
        }

        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(20, 24, 30));
        let d = DrawCtx {
            view: &self.view,
            camera: cam,
            rect,
            painter: &painter,
            ctx: ui.ctx(),
            time: ui.input(|i| i.time),
        };
        for layer in self.layers.iter_mut() {
            if self.view.layer_on(layer.id()) {
                layer.draw(&d);
            }
        }
        if self.view.layer_on("legend") {
            layers::draw_legend(&d, &self.layers);
        }
        out
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // Advance the clock, never spending more than ~40 ms of a frame on the model.
        if self.view.playing {
            let dt = ctx.input(|i| i.stable_dt).min(0.1) as f64;
            self.owed += dt * self.view.speed as f64;
            let start = web_time_now();
            while self.owed >= 1.0 && web_time_now() - start < 40.0 {
                self.apply(vec![Msg::Step(1)]);
                self.owed -= 1.0;
            }
            self.owed = self.owed.min(2.0);
            ctx.request_repaint();
        }

        if let Some(bytes) = PENDING_LOAD.with(|p| p.borrow_mut().take()) {
            self.apply(vec![Msg::Load(bytes)]);
        }
        let layer_names: Vec<(&'static str, &'static str)> =
            self.layers.iter().map(|l| (l.id(), l.name())).collect();
        let mut msgs = Vec::new();
        egui::Panel::left("panels")
            .resizable(true)
            .default_size(300.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for p in self.panels.iter_mut() {
                        egui::CollapsingHeader::new(p.title())
                            .default_open(p.default_open())
                            .show(ui, |ui| {
                                msgs.extend(p.ui(ui, &self.view, &layer_names));
                            });
                    }
                    if !self.view.status.is_empty() {
                        ui.separator();
                        ui.label(&self.view.status);
                    }
                });
            });
        egui::CentralPanel::default().show(ui, |ui| {
            msgs.extend(self.map_ui(ui));
        });
        self.apply(msgs);
    }
}

thread_local! {
    /// Bytes of a save file the user picked, waiting to be loaded next frame.
    static PENDING_LOAD: std::cell::RefCell<Option<Vec<u8>>> = const { std::cell::RefCell::new(None) };
}

/// Ask the user for a save file; its bytes arrive in `PENDING_LOAD`.
pub fn pick_save_file() {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let Ok(input) = doc
            .create_element("input")
            .map(|e| e.unchecked_into::<web_sys::HtmlInputElement>())
        else {
            return;
        };
        input.set_type("file");
        input.set_accept(".json");
        let inp = input.clone();
        let on_change = Closure::once(move || {
            let Some(file) = inp.files().and_then(|f| f.get(0)) else {
                return;
            };
            let Ok(reader) = web_sys::FileReader::new() else {
                return;
            };
            let r = reader.clone();
            let on_load = Closure::once(move || {
                if let Ok(text) = r.result() {
                    if let Some(s) = text.as_string() {
                        PENDING_LOAD.with(|p| *p.borrow_mut() = Some(s.into_bytes()));
                    }
                }
            });
            reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
            on_load.forget();
            let _ = reader.read_as_text(&file);
        });
        input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
        on_change.forget();
        input.click();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(bytes) = std::fs::read("hinterlands-save.json") {
            PENDING_LOAD.with(|p| *p.borrow_mut() = Some(bytes));
        }
    }
}

/// `?selftest=N` in the page URL.
fn selftest_years() -> Option<u32> {
    url_param("selftest")
}

/// A numeric `?name=N` from the page URL (`HINTERLANDS_<NAME>` natively).
fn url_param(name: &str) -> Option<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        let search = web_sys::window()?.location().search().ok()?;
        let prefix = format!("{name}=");
        search
            .trim_start_matches('?')
            .split('&')
            .find_map(|kv| kv.strip_prefix(prefix.as_str())?.parse().ok())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var(format!("HINTERLANDS_{}", name.to_uppercase()))
            .ok()?
            .parse()
            .ok()
    }
}

/// Milliseconds from a monotonic clock, on web and native.
fn web_time_now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0.0, |d| d.as_secs_f64() * 1000.0)
    }
}
