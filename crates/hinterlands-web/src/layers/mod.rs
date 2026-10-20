//! Map layers, drawn in order. A layer reads the snapshot and terrain and never
//! touches the simulation; adding one is a new file and a line in `all()`.

mod dots;
mod fields;
pub mod overlay_cost;
mod overlay_score;
mod routes;
mod settlements;
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
    /// Overlays are exclusive views toggled from the layer panel.
    fn draw(&mut self, d: &DrawCtx);
}

pub fn all() -> Vec<Box<dyn Layer>> {
    vec![
        Box::new(terrain::TerrainLayer::default()),
        Box::new(fields::FieldsLayer::default()),
        Box::new(overlay_cost::CostOverlay::default()),
        Box::new(overlay_score::ScoreOverlay::default()),
        Box::new(routes::RoutesLayer::default()),
        Box::new(settlements::SettlementsLayer::default()),
        Box::new(dots::DotsLayer::default()),
    ]
}
