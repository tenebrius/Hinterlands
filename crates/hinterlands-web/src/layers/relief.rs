//! Smooth relief: the base map drawn by a fragment shader that interpolates the
//! cells instead of painting squares. Coast, shallows, hillshade and the forest
//! canopy are derived per pixel from the cell values, and so are the farmed
//! countryside (field strips, pasture, hedgerows, old fields), the built-up
//! footprints of settlements with ruins and city walls, and the season tints,
//! each drawn only while its own layer is on. If the shader cannot be
//! built the layer falls back to the cell texture of `TerrainLayer`, and the
//! countryside layers draw their cells as before.

use super::countryside::{hash, strip_orientation};
use super::terrain::TerrainLayer;
use super::{DrawCtx, Layer, Swatch};
use crate::view::{ViewState, NEVER};
use eframe::egui_glow;
use eframe::glow::{self, HasContext};
use egui::Color32;
use hinterlands_core::state::{Rank, NONE};
use hinterlands_core::terrain::Terrain;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const VERT: &str = r#"
out vec2 v_uv;
void main() {
    vec2 p = vec2(float(gl_VertexID & 1), float((gl_VertexID >> 1) & 1)) * 2.0 - 1.0;
    v_uv = vec2(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5);
    gl_Position = vec4(p, 0.0, 1.0);
}
"#;

const FRAG: &str = include_str!("relief.frag");

/// How far the coast distance field is searched, in cells.
const COAST_REACH: i32 = 6;
/// Years of farming before a field is hedged, and years an abandoned field stays visible.
const ESTABLISHED: u32 = 50;
const GHOST_YEARS: u32 = 200;

/// Set when the shader could not be built, so the cell layers take over again.
static FAILED: AtomicBool = AtomicBool::new(false);

/// True while the shader draws the base map, fields, hedges and seasons; the
/// countryside layers then skip their per-cell versions of those.
pub fn active(v: &ViewState) -> bool {
    v.layer_on("relief") && !FAILED.load(Ordering::Relaxed)
}

#[derive(Default)]
pub struct ReliefLayer {
    gpu: Arc<Mutex<Gpu>>,
    key: Option<(u64, u32)>,
    terrain_key: Option<u64>,
    fallback: TerrainLayer,
}

/// GL objects, owned behind a mutex because egui runs paint callbacks later in the frame.
#[derive(Default)]
struct Gpu {
    program: Option<glow::Program>,
    vao: Option<glow::VertexArray>,
    land: Option<glow::Texture>,
    cover: Option<glow::Texture>,
    plot: Option<glow::Texture>,
    town: Option<glow::Texture>,
    /// Data waiting to be uploaded on the next paint.
    land_data: Option<Vec<f32>>,
    cover_data: Option<Vec<f32>>,
    plot_data: Option<Vec<f32>>,
    town_data: Option<Vec<f32>>,
    size: (i32, i32),
    failed: bool,
}

/// What the shader needs each frame.
#[derive(Clone, Copy)]
struct Frame {
    grid: [f32; 2],
    p0: [f32; 2],
    p1: [f32; 2],
    px: f32,
    year: f32,
    /// 0 spring .. 3 winter, or -1 with the seasons layer off.
    season: i32,
    /// The season the field colours follow, whether or not the seasons layer is on.
    field_season: i32,
    /// Which countryside layers are on: fields, strips, pasture, hedgerows and old fields.
    show: [f32; 4],
    /// Settlements and buildings layers on (footprints; walls).
    town: [f32; 2],
}

/// Per cell: elevation, signed distance to the coast in cells (land positive), yield, hillshade.
pub fn land_data(t: &Terrain) -> Vec<f32> {
    let g = t.grid;
    let (w, h) = (g.w as i32, g.h as i32);
    let sea = |x: i32, y: i32| t.is_sea(g.idx(x.clamp(0, w - 1), y.clamp(0, h - 1)));
    let cell_m = g.cell_km * 1000.0;
    let elev = |x: i32, y: i32| t.elevation[g.idx(x.clamp(0, w - 1), y.clamp(0, h - 1))].max(0.0);
    // Light from the north-west, 45 degrees up; relief exaggerated so 1 km cells read.
    let light = [-0.5f32, -0.5, std::f32::consts::FRAC_1_SQRT_2];
    let exaggerate = 12.0;
    let mut out = Vec::with_capacity(t.n() * 4);
    for y in 0..h {
        for x in 0..w {
            let i = g.idx(x, y);
            let here = sea(x, y);
            let mut best = COAST_REACH as f32;
            for dy in -COAST_REACH..=COAST_REACH {
                for dx in -COAST_REACH..=COAST_REACH {
                    if sea(x + dx, y + dy) != here {
                        best = best.min(((dx * dx + dy * dy) as f32).sqrt());
                    }
                }
            }
            let coast = (best - 0.5) * if here { -1.0 } else { 1.0 };
            // Horn's gradient over the 3 x 3 neighbourhood.
            let e = |dx: i32, dy: i32| elev(x + dx, y + dy);
            let dzdx = ((e(1, -1) + 2.0 * e(1, 0) + e(1, 1))
                - (e(-1, -1) + 2.0 * e(-1, 0) + e(-1, 1)))
                / (8.0 * cell_m);
            let dzdy = ((e(-1, 1) + 2.0 * e(0, 1) + e(1, 1))
                - (e(-1, -1) + 2.0 * e(0, -1) + e(1, -1)))
                / (8.0 * cell_m);
            let n = [-dzdx * exaggerate, -dzdy * exaggerate, 1.0];
            let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            let lambert = (n[0] * light[0] + n[1] * light[1] + n[2] * light[2]) / len;
            let shade = if here { 1.0 } else { lambert / light[2] };
            out.extend_from_slice(&[t.elevation[i], coast, t.food_yield[i], shade]);
        }
    }
    out
}

/// Per cell for this year: built up, ruined, inside a city's walls, and either
/// how fresh the ruin is or, on a built cell, whether its settlement is a town
/// or larger (which gets quays where it meets the sea).
fn town_data(v: &ViewState) -> Vec<f32> {
    let snap = &v.snapshot;
    let rank: std::collections::BTreeMap<u32, Rank> =
        snap.settlements.iter().map(|s| (s.id, s.rank)).collect();
    let mut out = Vec::with_capacity(snap.built.len() * 4);
    for (&b, &r) in snap.built.iter().zip(&snap.ruin) {
        let built = b != NONE;
        let ruin = !built && r > 0;
        let rank = rank.get(&b).copied();
        let walled = built && rank == Some(Rank::City);
        let last = if built {
            (rank >= Some(Rank::Town)) as u8 as f32
        } else {
            (r as f32 / 100.0).min(1.0)
        };
        out.extend_from_slice(&[
            built as u8 as f32,
            ruin as u8 as f32,
            walled as u8 as f32,
            last,
        ]);
    }
    out
}

/// Per cell for this year, two textures' worth.
/// Cover: forest, cultivated, pasture, abandoned-field fade.
/// Plot: owner (hashed to 0..1, -1 for none), strip direction plus 2 x rotation,
/// hedged (farmed for 50 years), flood meadow.
fn year_data(v: &ViewState) -> (Vec<f32>, Vec<f32>) {
    let t = &v.terrain;
    let g = t.grid;
    let snap = &v.snapshot;
    let year = snap.year;
    let horizontal = strip_orientation(v);
    let mut cover = Vec::with_capacity(t.n() * 4);
    let mut plot = Vec::with_capacity(t.n() * 4);
    for i in 0..t.n() {
        let built = snap.built[i] != NONE;
        let owned = snap.owner[i] != NONE;
        let forest = snap.forest[i] as f32 / 255.0;
        let farmed = owned && !built;
        let pasture = t.is_land(i)
            && !owned
            && !built
            && forest < 0.35
            && (0..8).any(|k| g.neighbor(i, k).is_some_and(|j| snap.owner[j] != NONE));
        let ghost = if !owned && !built && t.is_land(i) && v.farm_last[i] != NEVER {
            let age = year.saturating_sub(v.farm_last[i] as u32);
            if age == 0 || age > GHOST_YEARS {
                0.0
            } else {
                1.0 - age as f32 / GHOST_YEARS as f32
            }
        } else {
            0.0
        };
        cover.extend_from_slice(&[forest, farmed as u8 as f32, pasture as u8 as f32, ghost]);
        let owner = if farmed {
            (hash(snap.owner[i] as usize, 3) & 0xffff) as f32 / 65536.0
        } else {
            -1.0
        };
        let orient = horizontal[i] as u8 as f32 + 2.0 * (hash(i, 7) % 3) as f32;
        let hedged = farmed
            && v.farm_first[i] != NEVER
            && year.saturating_sub(v.farm_first[i] as u32) >= ESTABLISHED;
        let meadow = t.is_land(i) && t.slope[i] < 0.02 && t.touches(i, |j| t.is_nav(j));
        plot.extend_from_slice(&[owner, orient, hedged as u8 as f32, meadow as u8 as f32]);
    }
    (cover, plot)
}

impl Gpu {
    unsafe fn init(&mut self, gl: &glow::Context) -> Result<(), String> {
        let header = if gl.version().is_embedded {
            "#version 300 es\n"
        } else {
            "#version 330\n"
        };
        let program = gl.create_program()?;
        let mut shaders = Vec::new();
        for (kind, src) in [(glow::VERTEX_SHADER, VERT), (glow::FRAGMENT_SHADER, FRAG)] {
            let s = gl.create_shader(kind)?;
            gl.shader_source(s, &format!("{header}{src}"));
            gl.compile_shader(s);
            if !gl.get_shader_compile_status(s) {
                return Err(gl.get_shader_info_log(s));
            }
            gl.attach_shader(program, s);
            shaders.push(s);
        }
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            return Err(gl.get_program_info_log(program));
        }
        for s in shaders {
            gl.detach_shader(program, s);
            gl.delete_shader(s);
        }
        self.program = Some(program);
        self.vao = Some(gl.create_vertex_array()?);
        self.land = Some(gl.create_texture()?);
        self.cover = Some(gl.create_texture()?);
        self.plot = Some(gl.create_texture()?);
        self.town = Some(gl.create_texture()?);
        Ok(())
    }

    unsafe fn upload(gl: &glow::Context, tex: glow::Texture, size: (i32, i32), data: &[f32]) {
        gl.bind_texture(glow::TEXTURE_2D, Some(tex));
        for p in [glow::TEXTURE_MIN_FILTER, glow::TEXTURE_MAG_FILTER] {
            gl.tex_parameter_i32(glow::TEXTURE_2D, p, glow::NEAREST as i32);
        }
        for p in [glow::TEXTURE_WRAP_S, glow::TEXTURE_WRAP_T] {
            gl.tex_parameter_i32(glow::TEXTURE_2D, p, glow::CLAMP_TO_EDGE as i32);
        }
        gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 4);
        let bytes: Vec<u8> = data.iter().flat_map(|v| v.to_le_bytes()).collect();
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA32F as i32,
            size.0,
            size.1,
            0,
            glow::RGBA,
            glow::FLOAT,
            glow::PixelUnpackData::Slice(Some(&bytes)),
        );
    }

    fn paint(&mut self, gl: &glow::Context, f: Frame) {
        if self.failed {
            return;
        }
        unsafe {
            if self.program.is_none() {
                if let Err(e) = self.init(gl) {
                    log::error!("relief shader unavailable, drawing cells instead: {e}");
                    self.failed = true;
                    FAILED.store(true, Ordering::Relaxed);
                    return;
                }
            }
            let (Some(program), Some(land), Some(cover), Some(plot), Some(town)) =
                (self.program, self.land, self.cover, self.plot, self.town)
            else {
                return;
            };
            for (tex, data) in [
                (land, self.land_data.take()),
                (cover, self.cover_data.take()),
                (plot, self.plot_data.take()),
                (town, self.town_data.take()),
            ] {
                if let Some(data) = data {
                    Self::upload(gl, tex, self.size, &data);
                }
            }
            gl.use_program(Some(program));
            gl.bind_vertex_array(self.vao);
            for (unit, tex) in [
                (glow::TEXTURE3, town),
                (glow::TEXTURE2, plot),
                (glow::TEXTURE1, cover),
                (glow::TEXTURE0, land),
            ] {
                gl.active_texture(unit);
                gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            }
            let at = |name: &str| gl.get_uniform_location(program, name);
            gl.uniform_1_i32(at("u_land").as_ref(), 0);
            gl.uniform_1_i32(at("u_cover").as_ref(), 1);
            gl.uniform_1_i32(at("u_plot").as_ref(), 2);
            gl.uniform_1_i32(at("u_town").as_ref(), 3);
            gl.uniform_2_f32(at("u_town_show").as_ref(), f.town[0], f.town[1]);
            gl.uniform_1_f32(at("u_year").as_ref(), f.year);
            gl.uniform_1_i32(at("u_season").as_ref(), f.season);
            gl.uniform_1_i32(at("u_field_season").as_ref(), f.field_season);
            gl.uniform_4_f32(
                at("u_show").as_ref(),
                f.show[0],
                f.show[1],
                f.show[2],
                f.show[3],
            );
            gl.uniform_2_f32(at("u_grid").as_ref(), f.grid[0], f.grid[1]);
            gl.uniform_2_f32(at("u_p0").as_ref(), f.p0[0], f.p0[1]);
            gl.uniform_2_f32(at("u_p1").as_ref(), f.p1[0], f.p1[1]);
            gl.uniform_1_f32(at("u_px").as_ref(), f.px);
            gl.disable(glow::BLEND);
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            gl.enable(glow::BLEND);
            gl.bind_vertex_array(None);
        }
    }
}

impl Layer for ReliefLayer {
    fn id(&self) -> &'static str {
        "relief"
    }
    fn name(&self) -> &'static str {
        "Relief (smooth)"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (
                Swatch::Fill(Color32::from_rgb(163, 189, 115)),
                "Good farmland",
            ),
            (Swatch::Fill(Color32::from_rgb(204, 194, 153)), "Poor land"),
            (Swatch::Fill(Color32::from_rgb(61, 107, 56)), "Forest"),
            (
                Swatch::Fill(Color32::from_rgb(158, 153, 145)),
                "Upland rock",
            ),
            (Swatch::Fill(Color32::from_rgb(242, 242, 247)), "Snow"),
            (
                Swatch::Ramp(
                    Color32::from_rgb(153, 191, 204),
                    Color32::from_rgb(61, 102, 148),
                ),
                "Sea, shallow to deep",
            ),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let mut gpu = self.gpu.lock().unwrap_or_else(|e| e.into_inner());
        if gpu.failed {
            drop(gpu);
            self.fallback.draw(d);
            return;
        }
        if self.terrain_key != Some(v.terrain_version) {
            gpu.land_data = Some(land_data(t));
            gpu.size = (t.grid.w as i32, t.grid.h as i32);
            self.terrain_key = Some(v.terrain_version);
        }
        let key = (v.terrain_version, v.snapshot.year);
        if self.key != Some(key) {
            let (cover, plot) = year_data(v);
            gpu.cover_data = Some(cover);
            gpu.plot_data = Some(plot);
            gpu.town_data = Some(town_data(v));
            self.key = Some(key);
        }
        drop(gpu);

        let map = d.camera.map_rect(d.rect, t.grid.w, t.grid.h);
        let rect = map.intersect(d.rect);
        if !rect.is_positive() {
            return;
        }
        let p0 = d.camera.to_map(d.rect, rect.min);
        let p1 = d.camera.to_map(d.rect, rect.max);
        let frame = Frame {
            grid: [t.grid.w as f32, t.grid.h as f32],
            p0: [p0.x, p0.y],
            p1: [p1.x, p1.y],
            px: d.camera.zoom * d.ctx.pixels_per_point(),
            year: v.snapshot.year as f32,
            season: if v.layer_on("seasons") {
                v.season_index(d.time) as i32
            } else {
                -1
            },
            field_season: v.season_index(d.time) as i32,
            show: ["fields", "strips", "grazing", "history"].map(|id| v.layer_on(id) as u8 as f32),
            town: ["settlements", "buildings"].map(|id| v.layer_on(id) as u8 as f32),
        };
        let gpu = self.gpu.clone();
        d.painter.add(egui::PaintCallback {
            rect,
            callback: Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                gpu.lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .paint(painter.gl(), frame);
            })),
        });
    }
}
