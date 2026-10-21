//! Smooth relief: the base map drawn by a fragment shader that interpolates the
//! cells instead of painting squares. Coast, shallows, hillshade and the forest
//! canopy are derived per pixel from the cell values. If the shader cannot be
//! built the layer falls back to the cell texture of `TerrainLayer`.

use super::terrain::TerrainLayer;
use super::{DrawCtx, Layer, Swatch};
use eframe::egui_glow;
use eframe::glow::{self, HasContext};
use egui::Color32;
use hinterlands_core::terrain::Terrain;
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
    forest: Option<glow::Texture>,
    /// Data waiting to be uploaded on the next paint.
    land_data: Option<Vec<f32>>,
    forest_data: Option<Vec<f32>>,
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
        self.forest = Some(gl.create_texture()?);
        Ok(())
    }

    unsafe fn upload(
        gl: &glow::Context,
        tex: glow::Texture,
        size: (i32, i32),
        rgba: bool,
        data: &[f32],
    ) {
        let (internal, format) = if rgba {
            (glow::RGBA32F, glow::RGBA)
        } else {
            (glow::R32F, glow::RED)
        };
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
            internal as i32,
            size.0,
            size.1,
            0,
            format,
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
                    return;
                }
            }
            let (Some(program), Some(land), Some(forest)) = (self.program, self.land, self.forest)
            else {
                return;
            };
            if let Some(data) = self.land_data.take() {
                Self::upload(gl, land, self.size, true, &data);
            }
            if let Some(data) = self.forest_data.take() {
                Self::upload(gl, forest, self.size, false, &data);
            }
            gl.use_program(Some(program));
            gl.bind_vertex_array(self.vao);
            gl.active_texture(glow::TEXTURE1);
            gl.bind_texture(glow::TEXTURE_2D, Some(forest));
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(land));
            let at = |name: &str| gl.get_uniform_location(program, name);
            gl.uniform_1_i32(at("u_land").as_ref(), 0);
            gl.uniform_1_i32(at("u_forest").as_ref(), 1);
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
            gpu.forest_data = Some(
                v.snapshot
                    .forest
                    .iter()
                    .map(|&f| f as f32 / 255.0)
                    .collect(),
            );
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
