//! Map layers, drawn in order. A layer reads the snapshot and terrain and never
//! touches the simulation; adding one is a new file and a line in `all()`.

mod buildings;
mod countryside;
mod dots;
mod fields;
mod labels;
pub mod overlay_cost;
mod overlay_score;
mod parish;
mod routes;
mod settlements;
mod smoke;
pub mod terrain;

use crate::camera::Camera;
use crate::view::ViewState;
use egui::{Painter, Rect};

pub struct DrawCtx<'a> {
    pub view: &'a ViewState,
    pub camera: Camera,
    pub rect: Rect,
    pub painter: &'a Painter,
    pub ctx: &'a egui::Context,
    /// Seconds since start, for animation.
    pub time: f64,
}

pub trait Layer {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn default_on(&self) -> bool {
        true
    }
    fn draw(&mut self, d: &DrawCtx);
    /// Whether the layer draws anything at this zoom (for the legend).
    fn shown_at(&self, _zoom: f32) -> bool {
        true
    }
    /// Symbols this layer draws, for the legend.
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        Vec::new()
    }
}

/// How a legend entry is drawn.
#[derive(Clone, Copy)]
pub enum Swatch {
    Fill(egui::Color32),
    Line(egui::Color32, f32),
    Dot(egui::Color32),
    Ramp(egui::Color32, egui::Color32),
}

/// The legend's own toggle; the app draws it from the other visible layers.
pub struct LegendLayer;

impl Layer for LegendLayer {
    fn id(&self) -> &'static str {
        "legend"
    }
    fn name(&self) -> &'static str {
        "Legend"
    }
    fn draw(&mut self, _d: &DrawCtx) {}
}

/// Paint the legend for the visible layers in the map's lower-left corner.
pub fn draw_legend(d: &DrawCtx, layers: &[Box<dyn Layer>]) {
    use egui::{Color32, FontId, Rect, Stroke};
    let items: Vec<(Swatch, &str)> = layers
        .iter()
        .filter(|l| d.view.layer_on(l.id()) && l.shown_at(d.camera.zoom))
        .flat_map(|l| l.legend())
        .collect();
    if items.is_empty() {
        return;
    }
    let row = 16.0;
    let col_w = 190.0;
    let cols = if items.len() > 16 { 3 } else { 2 };
    let rows = items.len().div_ceil(cols);
    let size = egui::vec2(cols as f32 * col_w + 8.0, rows as f32 * row + 8.0);
    let box_rect = Rect::from_min_size(d.rect.right_bottom() - size - egui::vec2(10.0, 10.0), size);
    d.painter.rect_filled(
        box_rect,
        4.0,
        Color32::from_rgba_unmultiplied(250, 247, 240, 225),
    );
    d.painter.rect_stroke(
        box_rect,
        4.0,
        Stroke::new(1.0, Color32::from_gray(150)),
        egui::StrokeKind::Inside,
    );
    for (k, (sw, text)) in items.iter().enumerate() {
        let (c, r) = (k / rows, k % rows);
        let y = box_rect.top() + 4.0 + r as f32 * row + row / 2.0;
        let sx = box_rect.left() + 8.0 + c as f32 * col_w;
        let swatch = Rect::from_min_max(egui::pos2(sx, y - 5.0), egui::pos2(sx + 22.0, y + 5.0));
        match *sw {
            Swatch::Fill(c) => {
                d.painter.rect_filled(swatch, 2.0, c);
            }
            Swatch::Line(c, w) => {
                d.painter.line_segment(
                    [swatch.left_center(), swatch.right_center()],
                    Stroke::new(w, c),
                );
            }
            Swatch::Dot(c) => {
                d.painter.circle_filled(swatch.center(), 4.0, c);
            }
            Swatch::Ramp(a, b) => {
                for i in 0..11 {
                    let t = i as f32 / 10.0;
                    let mixc = Color32::from_rgb(
                        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
                        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
                        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
                    );
                    let x0 = swatch.left() + t * 20.0;
                    d.painter.rect_filled(
                        Rect::from_min_max(
                            egui::pos2(x0, swatch.top()),
                            egui::pos2(x0 + 2.0, swatch.bottom()),
                        ),
                        0.0,
                        mixc,
                    );
                }
            }
        }
        d.painter.text(
            egui::pos2(sx + 30.0, y),
            egui::Align2::LEFT_CENTER,
            *text,
            FontId::proportional(11.5),
            Color32::from_gray(40),
        );
    }
}

pub fn all() -> Vec<Box<dyn Layer>> {
    vec![
        Box::new(terrain::TerrainLayer::default()),
        Box::new(fields::FieldsLayer::default()),
        Box::new(countryside::CountrysideLayer::new(
            countryside::Kind::Grazing,
        )),
        Box::new(countryside::CountrysideLayer::new(
            countryside::Kind::Strips,
        )),
        Box::new(countryside::CountrysideLayer::new(
            countryside::Kind::Frontier,
        )),
        Box::new(parish::ParishLayer::new()),
        Box::new(parish::FootpathsLayer::new()),
        Box::new(overlay_cost::CostOverlay::default()),
        Box::new(overlay_score::ScoreOverlay::default()),
        Box::new(routes::RoutesLayer::default()),
        Box::new(settlements::SettlementsLayer::default()),
        Box::new(buildings::BuildingsLayer::default()),
        Box::new(dots::DotsLayer::default()),
        Box::new(smoke::SmokeLayer),
        Box::new(labels::LabelsLayer {
            class: labels::Class::Waters,
        }),
        Box::new(labels::LabelsLayer {
            class: labels::Class::Forests,
        }),
        Box::new(labels::LabelsLayer {
            class: labels::Class::Ranges,
        }),
        Box::new(labels::LabelsLayer {
            class: labels::Class::Rivers,
        }),
        Box::new(labels::LabelsLayer {
            class: labels::Class::Settlements,
        }),
        Box::new(LegendLayer),
    ]
}
