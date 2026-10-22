//! The regional grain store over time (S3, D81): the region's stores as a
//! share of a year's need, drawn under the population chart.

use super::charts::chart;
use crate::view::ViewState;
use egui::Color32;

pub fn ui(ui: &mut egui::Ui, v: &ViewState) {
    if !v.params.grain_stores {
        return;
    }
    let pts: Vec<[f32; 2]> = v.history.iter().map(|h| [h.year as f32, h.store]).collect();
    ui.weak("Grain in store, share of a year's need (gold)");
    // The zero line keeps the axis from starting at the lowest store.
    let floor: Vec<[f32; 2]> = match (v.history.first(), v.history.last()) {
        (Some(a), Some(b)) => vec![[a.year as f32, 0.0], [b.year as f32, 0.0]],
        _ => Vec::new(),
    };
    chart(
        ui,
        80.0,
        &[
            (&floor, Color32::TRANSPARENT, true),
            (&pts, crate::layers::GRAIN, true),
        ],
        ("year", "store / need"),
    );
}
