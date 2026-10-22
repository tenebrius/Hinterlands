//! Inspector: the settlement, cell and routes under the cursor (or the selected
//! cell when the cursor is off the map). Lookups live in `core::query`.

use super::Panel;
use crate::view::{Msg, ViewState};
use hinterlands_core::query;

pub struct Inspector;

impl Panel for Inspector {
    fn title(&self) -> &'static str {
        "Inspector"
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        _layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        let Some(cell) = v.hovered.or(v.selected) else {
            ui.weak("Hover over the map, or click to pin a cell.");
            return Vec::new();
        };
        let snap = &v.snapshot;
        let t = &v.terrain;
        let (x, y) = t.grid.xy(cell);
        if let Some(s) = query::settlement_near(snap, t, cell, 1.0) {
            let rank = 1 + snap.settlements.iter().filter(|o| o.pop > s.pop).count();
            ui.strong(format!("{} — {} (rank {rank})", s.name, s.rank.name()));
            egui::Grid::new("settlement").num_columns(2).show(ui, |ui| {
                ui.label("Population");
                ui.label(format!("{:.0}", s.pop));
                ui.end_row();
                ui.label("Founded");
                ui.label(if s.initial {
                    "start".to_string()
                } else {
                    format!("year {}", s.founded)
                });
                ui.end_row();
                ui.label("Food produced / needed");
                ui.label(format!("{:.0} / {:.0}", s.food_prod, s.need));
                ui.end_row();
                ui.label("Food in / out");
                ui.label(format!("{:.0} / {:.0}", s.food_in, s.food_out));
                ui.end_row();
                ui.label("Food ratio");
                ui.label(format!("{:.2}", s.food_ratio));
                ui.end_row();
                ui.label("Fields");
                ui.label(format!("{} cells", s.fields));
                ui.end_row();
                // Prices, purses and stores (v7): grain value units, one
                // person's grain for a year at price 1.
                if s.grain_price > 0.0 {
                    ui.label("Grain price");
                    ui.label(format!("{:.2}", s.grain_price));
                    ui.end_row();
                }
                if s.market > 0.0 {
                    ui.label("Market people");
                    ui.label(format!("{:.0}", s.market));
                    ui.end_row();
                }
                if s.purse > 0.0 || s.mpurse > 0.0 {
                    ui.label("Purses: farmers / market");
                    ui.label(format!("{:.0} / {:.0}", s.purse, s.mpurse));
                    ui.end_row();
                }
                if s.mstore > 0.0 {
                    ui.label("Market store");
                    ui.label(format!("{:.0} ({:.2} of a year's need)", s.mstore, s.mstore / s.need.max(1.0)));
                    ui.end_row();
                }
                if s.pasture > 0 {
                    ui.label("Pasture");
                    ui.label(format!("{} cells grazed for wool", s.pasture));
                    ui.end_row();
                }
                let trend = if s.pop_50_ago > 0.0 {
                    (s.pop / s.pop_50_ago - 1.0) * 100.0
                } else {
                    0.0
                };
                ui.label("50-year trend");
                ui.label(format!("{trend:+.0}% (from {:.0})", s.pop_50_ago));
                ui.end_row();
            });
            let partners = query::main_partners(snap, s.id, 3);
            if !partners.is_empty() {
                ui.label("Main trading partners:");
                for (id, f) in partners {
                    let (name, pop) = snap
                        .settlement(id)
                        .map_or((format!("#{id}"), 0.0), |o| (o.name.clone(), o.pop));
                    ui.label(format!("  {name} ({pop:.0} people): {f:.0}/yr"));
                }
            }
            ui.separator();
        }
        ui.strong(format!(
            "Cell ({x}, {y}) — {}",
            query::describe_cell(t, cell)
        ));
        for f in t
            .features
            .iter()
            .filter(|f| f.cells.contains(&(cell as u32)))
        {
            ui.label(&f.name);
        }
        ui.label(format!(
            "Elevation {:.0} m, yield {:.2}, forest {:.0}%",
            t.elevation[cell].max(0.0),
            t.food_yield[cell],
            snap.forest[cell] as f32 / 2.55
        ));
        if let Some(owner) = snap.settlement(snap.owner[cell]) {
            ui.label(format!("Field of {}", owner.name));
        }
        if t.is_land(cell) && v.selected == Some(cell) {
            let r = query::site_report(snap, t, &v.params, cell);
            if let Some((id, c)) = r.nearest {
                let name = snap
                    .settlement(id)
                    .map_or(format!("#{id}"), |o| o.name.clone());
                ui.label(format!(
                    "Travel cost to nearest settlement ({name}): {c:.1}"
                ));
            }
            let reasons: Vec<String> = r
                .parts
                .iter()
                .filter(|p| p.1.abs() > 0.005)
                .map(|(n, v)| format!("{n} {v:+.2}"))
                .collect();
            ui.label(format!(
                "Site score {:.2}: {}",
                r.total,
                if reasons.is_empty() {
                    "nothing".into()
                } else {
                    reasons.join(", ")
                }
            ));
            ui.weak(format!(
                "plus food ({:.1} prime cells free nearby) and noise",
                r.free_food
            ));
            if let Some(sc) = snap.site_score.get(cell).filter(|s| s.is_finite()) {
                ui.weak(format!("Score at the last founding step: {sc:.2}"));
            }
        } else if t.is_land(cell) {
            ui.weak("Click to see this cell's site score and travel cost.");
        }
        let routes = query::routes_through(snap, t, cell, 3);
        if !routes.is_empty() {
            ui.separator();
            ui.label("Routes through this cell:");
            let nm = |id: u32| {
                snap.settlement(id)
                    .map_or(format!("#{id}"), |o| o.name.clone())
            };
            for (a, b, f, water) in routes {
                let by = if water { "water" } else { "land" };
                ui.label(format!("  {} → {}: {f:.0}/yr by {by}", nm(a), nm(b)));
            }
        }
        Vec::new()
    }
}
