//! Base map: relief, water, land cover and forest, as one texture per year.

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, ColorImage, TextureHandle, TextureOptions};
use hinterlands_core::terrain::{Terrain, Water};

#[derive(Default)]
pub struct TerrainLayer {
    tex: Option<TextureHandle>,
    key: (u64, u32),
}

type Rgb = [f32; 3];

pub fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

pub fn to_color(c: Rgb) -> Color32 {
    Color32::from_rgb(
        (c[0].clamp(0.0, 1.0) * 255.0) as u8,
        (c[1].clamp(0.0, 1.0) * 255.0) as u8,
        (c[2].clamp(0.0, 1.0) * 255.0) as u8,
    )
}

fn shade(t: &Terrain, i: usize) -> f32 {
    let g = t.grid;
    let (x, y) = g.xy(i);
    let e = |xx: i32, yy: i32| {
        let xx = xx.clamp(0, g.w as i32 - 1);
        let yy = yy.clamp(0, g.h as i32 - 1);
        t.elevation[g.idx(xx, yy)].max(0.0)
    };
    let dz = (e(x - 1, y - 1) - e(x + 1, y + 1)) / (g.cell_km * 1000.0);
    (1.0 + dz * 4.0).clamp(0.6, 1.3)
}

/// Colour of a cell given its forest cover.
pub fn cell_color(t: &Terrain, i: usize, forest: f32) -> Rgb {
    match t.water[i] {
        Water::Sea => mix(
            [0.18, 0.34, 0.56],
            [0.06, 0.14, 0.32],
            (-t.elevation[i] / 300.0).clamp(0.0, 1.0),
        ),
        Water::NavigableRiver => [0.1, 0.3, 0.78],
        _ => {
            let mut c = mix([0.74, 0.68, 0.52], [0.56, 0.73, 0.36], t.food_yield[i]);
            c = mix(
                c,
                [0.88, 0.88, 0.88],
                ((t.elevation[i] - 900.0) / 400.0).clamp(0.0, 1.0),
            );
            c = mix(c, [0.16, 0.38, 0.18], forest * 0.85);
            if t.water[i] == Water::Stream {
                c = mix(c, [0.38, 0.58, 0.88], 0.7);
            }
            let s = shade(t, i);
            [c[0] * s, c[1] * s, c[2] * s]
        }
    }
}

impl Layer for TerrainLayer {
    fn id(&self) -> &'static str {
        "terrain"
    }
    fn name(&self) -> &'static str {
        "Terrain and forest"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (
                Swatch::Fill(Color32::from_rgb(143, 186, 92)),
                "Good farmland",
            ),
            (Swatch::Fill(Color32::from_rgb(189, 173, 133)), "Poor land"),
            (Swatch::Fill(Color32::from_rgb(52, 104, 52)), "Forest"),
            (Swatch::Fill(Color32::from_rgb(224, 224, 224)), "Mountains"),
            (Swatch::Fill(Color32::from_rgb(97, 148, 224)), "Stream"),
            (
                Swatch::Fill(Color32::from_rgb(26, 77, 199)),
                "Navigable river",
            ),
            (Swatch::Fill(Color32::from_rgb(46, 87, 143)), "Sea"),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let key = (v.terrain_version, v.snapshot.year);
        if self.tex.is_none() || self.key != key {
            let n = t.n();
            let mut px = Vec::with_capacity(n);
            for i in 0..n {
                px.push(to_color(cell_color(
                    t,
                    i,
                    v.snapshot.forest[i] as f32 / 255.0,
                )));
            }
            let img = ColorImage::new([t.grid.w as usize, t.grid.h as usize], px);
            match &mut self.tex {
                Some(tex) => tex.set(img, TextureOptions::NEAREST),
                None => {
                    self.tex = Some(d.ctx.load_texture("terrain", img, TextureOptions::NEAREST))
                }
            }
            self.key = key;
        }
        if let Some(tex) = &self.tex {
            let r = d.camera.map_rect(d.rect, t.grid.w, t.grid.h);
            d.painter.image(
                tex.id(),
                r,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
    }
}
