//! Rivers as lines: each stream and river follows its drainage from cell to
//! cell, smoothed, widening with the water it carries. Built once per terrain.

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, Pos2, Rect, Shape, Stroke, Vec2};
use hinterlands_core::terrain::{Terrain, Water};

const STREAM: Color32 = Color32::from_rgb(74, 124, 186);
const NAVIGABLE: Color32 = Color32::from_rgb(44, 88, 168);

/// One unbroken run of river between sources, confluences and the sea.
struct Chain {
    /// Map points in cells.
    pts: Vec<Vec2>,
    /// Channel width at each point, in cells.
    width: Vec<f32>,
    navigable: Vec<bool>,
    bounds: Rect,
}

#[derive(Default)]
pub struct RiversLayer {
    chains: Vec<Chain>,
    key: Option<u64>,
}

/// Channel width in km for a drainage flux: wide on the lower river, a thread at the source.
fn width_km(flux: f32) -> f32 {
    (0.012 * flux.max(1.0).powf(0.4)).min(0.45)
}

fn chains(t: &Terrain) -> Vec<Chain> {
    let g = t.grid;
    let n = t.n();
    let centre = |i: usize| {
        let (x, y) = g.xy(i);
        Vec2::new(x as f32 + 0.5, y as f32 + 0.5)
    };
    let mut inflow = vec![0u8; n];
    for i in 0..n {
        if t.is_river(i) {
            if let Some(j) = t.downstream(i) {
                inflow[j] = inflow[j].saturating_add(1);
            }
        }
    }
    let mut out = Vec::new();
    for start in (0..n).filter(|&i| t.is_river(i) && inflow[i] != 1) {
        let mut pts = vec![centre(start)];
        let mut cells = vec![start];
        let mut cur = start;
        while let Some(j) = t.downstream(cur) {
            if t.is_sea(j) {
                // End at the shore, between the last river cell and the sea.
                pts.push((centre(cur) + centre(j)) * 0.5);
                cells.push(cur);
                break;
            }
            pts.push(centre(j));
            cells.push(j);
            if !t.is_river(j) || inflow[j] != 1 {
                break;
            }
            cur = j;
        }
        if pts.len() < 2 {
            continue;
        }
        let width = cells
            .iter()
            .map(|&i| width_km(t.flux[i]) / g.cell_km)
            .collect();
        let navigable = cells
            .iter()
            .map(|&i| t.water[i] == Water::NavigableRiver)
            .collect();
        out.push(smooth(Chain {
            pts,
            width,
            navigable,
            bounds: Rect::NOTHING,
        }));
    }
    out
}

/// Two rounds of Chaikin corner cutting, endpoints kept so chains still meet.
fn smooth(mut c: Chain) -> Chain {
    for _ in 0..2 {
        let k = c.pts.len();
        if k < 3 {
            break;
        }
        let mut pts = vec![c.pts[0]];
        let mut width = vec![c.width[0]];
        let mut nav = vec![c.navigable[0]];
        for s in 0..k - 1 {
            let (a, b) = (c.pts[s], c.pts[s + 1]);
            let (wa, wb) = (c.width[s], c.width[s + 1]);
            for f in [0.25, 0.75] {
                if (s == 0 && f == 0.25) || (s == k - 2 && f == 0.75) {
                    continue;
                }
                pts.push(a + (b - a) * f);
                width.push(wa + (wb - wa) * f);
                nav.push(if f < 0.5 {
                    c.navigable[s]
                } else {
                    c.navigable[s + 1]
                });
            }
        }
        pts.push(c.pts[k - 1]);
        width.push(c.width[k - 1]);
        nav.push(c.navigable[k - 1]);
        c = Chain {
            pts,
            width,
            navigable: nav,
            bounds: Rect::NOTHING,
        };
    }
    let mut bounds = Rect::NOTHING;
    for p in &c.pts {
        bounds.extend_with(Pos2::new(p.x, p.y));
    }
    c.bounds = bounds.expand(0.5);
    c
}

impl Layer for RiversLayer {
    fn id(&self) -> &'static str {
        "rivers"
    }
    fn name(&self) -> &'static str {
        "Rivers (smooth)"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (Swatch::Line(STREAM, 1.5), "Stream"),
            (Swatch::Line(NAVIGABLE, 3.0), "Navigable river"),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        if self.key != Some(v.terrain_version) {
            self.chains = chains(t);
            self.key = Some(v.terrain_version);
        }
        let z = d.camera.zoom;
        // Spring high water widens every channel.
        let flood = if v.layer_on("seasons") && v.season_index(d.time) == 0 {
            1.5
        } else {
            1.0
        };
        // Far out, the smallest streams thin to hairlines and fade so the map is not a web.
        let stream_alpha = (z / 6.0).clamp(0.45, 1.0);
        let view = Rect::from_min_max(
            d.camera.to_map(d.rect, d.rect.min).to_pos2(),
            d.camera.to_map(d.rect, d.rect.max).to_pos2(),
        );
        for c in &self.chains {
            if !c.bounds.intersects(view) {
                continue;
            }
            // Runs of points with the same drawn width and kind become one polyline.
            let mut run: Vec<Pos2> = Vec::new();
            let mut style: Option<(u32, bool)> = None;
            for k in 0..c.pts.len() {
                let px = (c.width[k] * z * flood).max(if c.navigable[k] { 1.6 } else { 0.9 });
                let s = ((px * 2.0).round() as u32, c.navigable[k]);
                let p = d.camera.to_screen(d.rect, c.pts[k]);
                if style.is_some_and(|old| old != s) {
                    run.push(p);
                    flush(d, &mut run, style, stream_alpha);
                }
                style = Some(s);
                run.push(p);
            }
            flush(d, &mut run, style, stream_alpha);
        }
    }
}

fn flush(d: &DrawCtx, run: &mut Vec<Pos2>, style: Option<(u32, bool)>, stream_alpha: f32) {
    let Some((half_px, nav)) = style else {
        run.clear();
        return;
    };
    if run.len() >= 2 {
        let w = half_px as f32 / 2.0;
        let colour = if nav {
            NAVIGABLE
        } else {
            STREAM.gamma_multiply(stream_alpha)
        };
        if w > 2.5 {
            // Round the joints of wide channels.
            for p in run.iter() {
                d.painter.circle_filled(*p, w / 2.0, colour);
            }
        }
        d.painter
            .add(Shape::line(std::mem::take(run), Stroke::new(w, colour)));
    }
    run.clear();
}
