//! Seasons (1.x layer; drawing only): high water on rivers and floodplains in
//! spring, autumn colour in the woods, frost and snow on high ground in winter.
//! Field colours and winter shipping follow the same season (see the field
//! strips and trade traffic layers).

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, ColorImage, TextureHandle, TextureOptions};

#[derive(Default)]
pub struct SeasonsLayer {
    tex: Option<TextureHandle>,
    key: Option<(u64, u32, usize, bool)>,
}

impl Layer for SeasonsLayer {
    fn id(&self) -> &'static str {
        "seasons"
    }
    fn name(&self) -> &'static str {
        "Seasons"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (
                Swatch::Fill(Color32::from_rgb(120, 170, 235)),
                "Spring high water",
            ),
            (
                Swatch::Fill(Color32::from_rgb(235, 240, 245)),
                "Winter frost and snow",
            ),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let season = v.season_index(d.time);
        // The relief shader applies the season itself, per pixel.
        if super::relief::active(v) {
            if v.season_fixed.is_none() && !v.playing {
                d.ctx.request_repaint();
            }
            return;
        }
        // With the river lines on, spring high water widens them instead of tinting cells.
        let rivers = v.layer_on("rivers");
        let key = (v.terrain_version, v.snapshot.year, season, rivers);
        if self.key != Some(key) {
            let px = (0..t.n())
                .map(|i| {
                    let forest = v.snapshot.forest[i] as f32 / 255.0;
                    match season {
                        0 if t.is_river(i) && !rivers => {
                            Color32::from_rgba_unmultiplied(120, 180, 245, 150)
                        }
                        0 if t.is_land(i) && t.slope[i] < 0.02 && t.touches(i, |j| t.is_nav(j)) => {
                            Color32::from_rgba_unmultiplied(120, 170, 235, 70)
                        }
                        2 if t.is_land(i) && forest > 0.5 => {
                            Color32::from_rgba_unmultiplied(200, 120, 40, (60.0 * forest) as u8)
                        }
                        3 if t.is_land(i) && t.elevation[i] > 750.0 => {
                            Color32::from_rgba_unmultiplied(245, 248, 252, 170)
                        }
                        3 if t.is_land(i) => Color32::from_rgba_unmultiplied(235, 240, 245, 55),
                        _ => Color32::TRANSPARENT,
                    }
                })
                .collect();
            let img = ColorImage::new([t.grid.w as usize, t.grid.h as usize], px);
            match &mut self.tex {
                Some(tex) => tex.set(img, TextureOptions::NEAREST),
                None => {
                    self.tex = Some(d.ctx.load_texture("seasons", img, TextureOptions::NEAREST))
                }
            }
            self.key = Some(key);
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
        if v.season_fixed.is_none() && !v.playing {
            d.ctx.request_repaint();
        }
    }
}
