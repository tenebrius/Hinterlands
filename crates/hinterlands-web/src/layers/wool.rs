//! Wool (L4; drawing only): the model's pasture (L1) for the countryside
//! layers, and wool going down to the outside ports as a seventh colour of
//! route, fleece cream.

use super::lines::chaikin;
use super::{DrawCtx, Layer, Swatch};
use crate::view::ViewState;
use egui::{vec2, Color32, Pos2, Shape, Stroke, Vec2};
use hinterlands_core::snapshot::Snapshot;
use hinterlands_core::state::NONE;
use hinterlands_core::terrain::Terrain;

pub const WOOL: Color32 = Color32::from_rgb(214, 200, 160);
const WOOL_EDGE: Color32 = Color32::from_rgb(120, 104, 70);
/// Most wool flows drawn.
const MAX_DRAWN: usize = 40;

/// Which cells are pasture: the model's grazed land (L1). A snapshot from
/// before L1 carries none, and then open land by the fields is drawn as
/// grazing, as the view did before.
pub fn pasture_mask(snap: &Snapshot, t: &Terrain) -> Vec<bool> {
    let mut mask = vec![false; t.n()];
    if !snap.pasture.is_empty() {
        for &c in &snap.pasture {
            if let Some(m) = mask.get_mut(c as usize) {
                *m = true;
            }
        }
        return mask;
    }
    let g = t.grid;
    for (i, m) in mask.iter_mut().enumerate() {
        *m = t.is_land(i)
            && snap.owner[i] == NONE
            && snap.built[i] == NONE
            && (snap.forest[i] as f32 / 255.0) < 0.35
            && (0..8).any(|k| g.neighbor(i, k).is_some_and(|j| snap.owner[j] != NONE));
    }
    mask
}

/// True if the model's own land use is shown (a snapshot from L1 on).
pub fn modelled(v: &ViewState) -> bool {
    !v.snapshot.pasture.is_empty()
}

#[derive(Default)]
pub struct WoolLayer;

impl Layer for WoolLayer {
    fn id(&self) -> &'static str {
        "wool"
    }
    fn name(&self) -> &'static str {
        "Wool to the ports"
    }
    fn legend(&self) -> Vec<(Swatch, &'static str)> {
        vec![(Swatch::Line(WOOL, 3.0), "Wool carried to the ports")]
    }
    fn draw(&mut self, d: &DrawCtx) {
        let v = d.view;
        let snap = &v.snapshot;
        let g = v.terrain.grid;
        let n = v.terrain.n() as u32;
        let z = d.camera.zoom;
        let on_map = |node: u32| -> Vec2 {
            let c = node % n;
            let c = v.sea_lane.get(c as usize).copied().unwrap_or(c);
            let (x, y) = g.xy(c as usize);
            vec2(x as f32 + 0.5, y as f32 + 0.5)
        };
        let mut flows: Vec<_> = snap.flows.iter().filter(|f| f.wool).collect();
        flows.sort_by(|a, b| b.amount.total_cmp(&a.amount));
        flows.truncate(MAX_DRAWN);
        flows.reverse();
        for f in flows {
            if f.path.len() < 2 {
                continue;
            }
            let pts: Vec<Vec2> = f.path.iter().map(|&c| on_map(c)).collect();
            let (smooth, _) = chaikin(&pts, &vec![(); pts.len()], 2, |a, _, _| a);
            let line: Vec<Pos2> = smooth
                .iter()
                .map(|&p| d.camera.to_screen(d.rect, p))
                .collect();
            let w = (0.4 + f.amount.sqrt() * 0.08).min(3.5) * (z / 6.0).clamp(0.6, 2.0);
            d.painter
                .add(Shape::line(line.clone(), Stroke::new(w + 1.2, WOOL_EDGE.gamma_multiply(0.5))));
            d.painter.add(Shape::line(line, Stroke::new(w, WOOL)));
        }
    }
}
