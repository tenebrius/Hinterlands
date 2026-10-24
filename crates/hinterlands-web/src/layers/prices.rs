//! Grain prices (v7 release; drawing only): a disc at each place with a
//! market price, coloured from cheap (blue) through the world price (pale) to
//! dear (red), sized by population. Prices move together across the region,
//! so the colour reads the region's level against the world as well as the
//! spread between places.

use super::{DrawCtx, Layer, Swatch};
use egui::{Color32, Stroke};

const CHEAP: Color32 = Color32::from_rgb(46, 98, 168);
const MID: Color32 = Color32::from_rgb(236, 230, 210);
const DEAR: Color32 = Color32::from_rgb(178, 40, 36);
const INK: Color32 = Color32::from_rgb(40, 34, 30);

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let f = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(f(a.r(), b.r()), f(a.g(), b.g()), f(a.b(), b.b()))
}

#[derive(Default)]
pub struct PricesLayer;

impl Layer for PricesLayer {
    fn id(&self) -> &'static str {
        "prices"
    }
    fn name(&self) -> &'static str {
        "Grain prices"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![
            (
                Swatch::Fill(CHEAP),
                "Grain cheap (a quarter of the world price)",
            ),
            (Swatch::Fill(MID), "Grain at the world price"),
            (
                Swatch::Fill(DEAR),
                "Grain dear (four times the world price)",
            ),
        ]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let snap = &v.snapshot;
        let world = v.params.port_world_price.max(1e-3);
        let g = v.terrain.grid;
        let z = d.camera.zoom;
        // A disc per place is a map symbol: close in it fades with the others,
        // so the market place it marks can be seen.
        let fade = crate::scale::symbol_alpha(z, v.layer_on("symbols"));
        if fade <= 0.0 {
            return;
        }
        for s in &snap.settlements {
            if s.grain_price <= 0.0 {
                continue;
            }
            // log2 of price over the world price, -2 (a quarter) to +2 (four times).
            let k = ((s.grain_price / world).log2() / 2.0).clamp(-1.0, 1.0);
            let fill = if k < 0.0 {
                mix(MID, CHEAP, -k)
            } else {
                mix(MID, DEAR, k)
            };
            let c = d.camera.cell_center(d.rect, g.w, s.anchor as usize);
            let r = (3.0 + s.pop.sqrt() * 0.06).min(12.0) * (z / 6.0).clamp(0.7, 2.0);
            d.painter.circle(
                c,
                r,
                fill.gamma_multiply(fade),
                Stroke::new(0.8, INK.gamma_multiply(fade)),
            );
        }
    }
}
