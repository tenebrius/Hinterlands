//! Trade animated as moving dots. Yearly flow becomes trips per year (cargo per
//! trip by mode: cart, river barge, sea ship); dots spawn at that rate and move
//! along the stored path at each mode's speed. One real second is one day.
//! Dots are capped per route, with more cargo per dot when flows are large.
//! The simulation never sees the dots.

use super::{DrawCtx, Layer};
use egui::{Color32, Pos2};
use hinterlands_core::terrain::Terrain;

/// Tonnes of goods per food unit (a person's grain for a year, about 250 kg).
const TONNES_PER_UNIT: f32 = 0.25;
const MAX_DOTS_PER_ROUTE: f32 = 5.0;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Cart,
    Barge,
    Ship,
}

impl Mode {
    fn km_per_day(self) -> f32 {
        match self {
            Mode::Cart => 30.0,
            Mode::Barge => 40.0,
            Mode::Ship => 100.0,
        }
    }
    fn tonnes(self) -> f32 {
        match self {
            Mode::Cart => 1.0,
            Mode::Barge => 25.0,
            Mode::Ship => 100.0,
        }
    }
    fn color(self) -> Color32 {
        match self {
            Mode::Cart => Color32::from_rgb(250, 220, 120),
            Mode::Barge => Color32::from_rgb(120, 230, 255),
            Mode::Ship => Color32::WHITE,
        }
    }
}

struct Leg {
    a: Pos2,
    b: Pos2,
    /// Days from the route's start to this leg's start, and the leg's length in days.
    t0: f32,
    dt: f32,
    mode: Mode,
}

struct Route {
    legs: Vec<Leg>,
    days: f32,
    /// Days between dots.
    every: f32,
    phase: f32,
}

#[derive(Default)]
pub struct DotsLayer {
    routes: Vec<Route>,
    key: Option<(u64, u32, i64, i64, i64)>,
}

fn mode_of(t: &Terrain, n: u32, a: u32, b: u32) -> Mode {
    if a >= n && b >= n {
        let ca = (a - n) as usize;
        let cb = (b - n) as usize;
        if t.is_sea(ca) || t.is_sea(cb) {
            Mode::Ship
        } else {
            Mode::Barge
        }
    } else {
        Mode::Cart
    }
}

impl Layer for DotsLayer {
    fn id(&self) -> &'static str {
        "dots"
    }
    fn name(&self) -> &'static str {
        "Trade traffic"
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let t = &v.terrain;
        let g = t.grid;
        let n = t.n() as u32;
        // Positions are cached in screen space, so the camera is part of the key.
        let key = (
            v.terrain_version,
            v.snapshot.year,
            (d.camera.zoom * 100.0) as i64,
            (d.camera.center.x * 100.0) as i64,
            (d.camera.center.y * 100.0) as i64,
        );
        if self.key != Some(key) {
            self.routes.clear();
            let mut flows: Vec<_> = v
                .snapshot
                .flows
                .iter()
                .filter(|f| f.path.len() > 1)
                .collect();
            flows.sort_by(|a, b| b.amount.total_cmp(&a.amount));
            for f in flows.into_iter().take(400) {
                let mut legs = Vec::new();
                let mut t0 = 0.0;
                let mut cargo = 1.0f32;
                for w in f.path.windows(2) {
                    let (ca, cb) = ((w[0] % n) as usize, (w[1] % n) as usize);
                    if ca == cb {
                        continue;
                    }
                    let mode = mode_of(t, n, w[0], w[1]);
                    cargo = cargo.max(mode.tonnes());
                    let dt = g.dist_km(ca, cb) / mode.km_per_day();
                    legs.push(Leg {
                        a: d.camera.cell_center(d.rect, g.w, ca),
                        b: d.camera.cell_center(d.rect, g.w, cb),
                        t0,
                        dt,
                        mode,
                    });
                    t0 += dt;
                }
                if legs.is_empty() || t0 <= 0.0 {
                    continue;
                }
                let trips = f.amount * TONNES_PER_UNIT / cargo;
                if trips <= 0.0 {
                    continue;
                }
                let every = (365.0 / trips).max(t0 / MAX_DOTS_PER_ROUTE);
                let phase = ((f.from.wrapping_mul(2654435761) ^ f.to.wrapping_mul(40503)) % 1000)
                    as f32
                    / 1000.0
                    * every;
                self.routes.push(Route {
                    legs,
                    days: t0,
                    every,
                    phase,
                });
            }
            self.key = Some(key);
        }
        let now = d.time as f32;
        for r in &self.routes {
            let mut s = (now + r.phase) % r.every;
            while s < r.days {
                if let Some(leg) = r.legs.iter().find(|l| s >= l.t0 && s < l.t0 + l.dt) {
                    let u = (s - leg.t0) / leg.dt.max(1e-6);
                    let p = leg.a + (leg.b - leg.a) * u;
                    if d.rect.contains(p) {
                        d.painter.circle_filled(
                            p,
                            (0.12 * d.camera.zoom).clamp(1.2, 3.5),
                            leg.mode.color(),
                        );
                    }
                }
                s += r.every;
            }
        }
        d.ctx.request_repaint();
    }
}
