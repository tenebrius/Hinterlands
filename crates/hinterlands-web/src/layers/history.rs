//! Hedgerows and ghost fields (1.x layer; drawing only), from the farming
//! history the viewer records as it steps through the years: fields farmed for
//! fifty years or more are bounded by hedges; land that has gone out of
//! cultivation keeps a fading pattern of its old strips and hedges for two
//! centuries, so dead villages leave a ghost on the map.

use super::{DrawCtx, Layer, Swatch};
use crate::view::NEVER;
use egui::{Color32, Stroke};
use hinterlands_core::state::NONE;

const HEDGE: Color32 = Color32::from_rgb(44, 92, 40);
const ESTABLISHED: u32 = 50;
const GHOST_YEARS: u32 = 200;

pub struct HistoryLayer;

impl Layer for HistoryLayer {
    fn id(&self) -> &'static str {
        "history"
    }
    fn name(&self) -> &'static str {
        "Hedgerows and old fields"
    }
    fn shown_at(&self, zoom: f32) -> bool {
        zoom >= 8.0
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (Swatch::Line(HEDGE, 2.0), "Hedgerow"),
            (
                Swatch::Line(Color32::from_rgba_unmultiplied(90, 80, 60, 120), 1.0),
                "Abandoned fields",
            ),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let z = d.camera.zoom;
        if z < 8.0 {
            return;
        }
        let v = d.view;
        let t = &v.terrain;
        let g = t.grid;
        let snap = &v.snapshot;
        let year = snap.year;
        let (lo, hi) = (
            d.camera.to_map(d.rect, d.rect.min),
            d.camera.to_map(d.rect, d.rect.max),
        );
        let (x0, y0) = ((lo.x.floor() as i32).max(0), (lo.y.floor() as i32).max(0));
        let (x1, y1) = (
            (hi.x.ceil() as i32).min(g.w as i32 - 1),
            (hi.y.ceil() as i32).min(g.h as i32 - 1),
        );
        let hedge = Stroke::new((0.06 * z).clamp(1.2, 3.0), HEDGE);
        let edges = [
            (0usize, egui::vec2(0.5, -0.5), egui::vec2(0.5, 0.5)),
            (2, egui::vec2(-0.5, 0.5), egui::vec2(0.5, 0.5)),
            (4, egui::vec2(-0.5, -0.5), egui::vec2(-0.5, 0.5)),
            (6, egui::vec2(-0.5, -0.5), egui::vec2(0.5, -0.5)),
        ];
        for y in y0..=y1 {
            for x in x0..=x1 {
                let i = g.idx(x, y);
                if snap.built[i] != NONE {
                    continue;
                }
                let c = d.camera.cell_center(d.rect, g.w, i);
                let farmed_now = snap.owner[i] != NONE;
                if farmed_now
                    && v.farm_first[i] != NEVER
                    && year.saturating_sub(v.farm_first[i] as u32) >= ESTABLISHED
                {
                    // Hedges where the field meets anything but another field of the same owner.
                    for &(k, a, b) in &edges {
                        let same = g
                            .neighbor(i, k)
                            .is_some_and(|j| snap.owner[j] == snap.owner[i]);
                        if !same {
                            d.painter
                                .line_segment([c + a * z * 0.96, c + b * z * 0.96], hedge);
                        }
                    }
                } else if !farmed_now && v.farm_last[i] != NEVER {
                    let age = year.saturating_sub(v.farm_last[i] as u32);
                    if age == 0 || age > GHOST_YEARS || !t.is_land(i) {
                        continue;
                    }
                    let fade = 1.0 - age as f32 / GHOST_YEARS as f32;
                    let ghost = Stroke::new(
                        1.0,
                        Color32::from_rgba_unmultiplied(90, 80, 60, (40.0 + 100.0 * fade) as u8),
                    );
                    // The old strips, then the old hedge line around them.
                    for k in 1..4 {
                        let u = k as f32 / 4.0 - 0.5;
                        d.painter.line_segment(
                            [c + egui::vec2(-0.45, u) * z, c + egui::vec2(0.45, u) * z],
                            ghost,
                        );
                    }
                    let old_hedge = Stroke::new(
                        1.5,
                        Color32::from_rgba_unmultiplied(44, 92, 40, (50.0 + 110.0 * fade) as u8),
                    );
                    for &(k, a, b) in &edges {
                        let inside = g
                            .neighbor(i, k)
                            .is_some_and(|j| snap.owner[j] == NONE && v.farm_last[j] != NEVER);
                        if !inside {
                            d.painter
                                .line_segment([c + a * z * 0.96, c + b * z * 0.96], old_hedge);
                        }
                    }
                }
            }
        }
    }
}
