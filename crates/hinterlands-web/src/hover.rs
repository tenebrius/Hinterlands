//! The hover card: what lies under the cursor, from the top of the map down,
//! for the layers that are on. It reads `ViewState` only, like a panel.

use crate::view::{ViewState, NEVER};
use hinterlands_core::query;
use hinterlands_core::state::NONE;

pub fn card(ui: &mut egui::Ui, v: &ViewState, cell: usize) {
    ui.set_max_width(280.0);
    let snap = &v.snapshot;
    let t = &v.terrain;
    let on = |id: &str| v.layer_on(id);
    let name = |id: u32| {
        snap.settlement(id)
            .map_or_else(|| format!("#{id}"), |s| s.name.clone())
    };
    let heading = |ui: &mut egui::Ui, text: String| {
        ui.label(
            egui::RichText::new(text)
                .font(crate::style::serif(17.0))
                .color(crate::style::INK),
        );
    };

    let near = on("settlements")
        .then(|| query::settlement_near(snap, t, cell, 1.0))
        .flatten();
    if let Some(s) = near {
        heading(
            ui,
            format!("{} — {}, {:.0} people", s.name, s.rank.name(), s.pop),
        );
        if s.pop_50_ago > 0.0 {
            let change = (s.pop / s.pop_50_ago - 1.0) * 100.0;
            let word = if change >= 2.0 {
                "Growing"
            } else if change <= -2.0 {
                "Shrinking"
            } else {
                "Steady"
            };
            ui.label(format!("{word}: {change:+.0}% in the last 50 years."));
        }
        if s.food_ratio < 1.0 {
            ui.label(format!(
                "Short of food: it has {:.0}% of what it needs.",
                s.food_ratio * 100.0
            ));
        }
        if s.need > 0.0 && s.food_in > 0.3 * s.need {
            ui.label(format!(
                "Buys {:.0}% of its food at market.",
                (s.food_in / s.need * 100.0).min(100.0)
            ));
        }
        if s.through > 200.0 {
            ui.label(format!(
                "Trade crossroads: {:.0} a year passes through.",
                s.through
            ));
        }
        ui.weak(if s.initial {
            "One of the first settlements.".to_string()
        } else {
            format!("Founded in year {}.", s.founded)
        });
        ui.separator();
    }

    let mut lines: Vec<String> = Vec::new();
    let forest = snap.forest[cell] as f32 / 255.0;
    if on("settlements") && near.is_none() {
        if snap.built[cell] != NONE {
            lines.push(format!("Houses and streets of {}.", name(snap.built[cell])));
        } else if snap.ruin[cell] > 0 {
            lines.push("Ruins of an abandoned settlement.".into());
        }
    }
    let owner = snap.owner[cell];
    if owner != NONE && snap.built[cell] == NONE && (on("fields") || on("strips")) {
        let mut s = format!("Fields of {}", name(owner));
        if v.farm_first[cell] != NEVER {
            let since = v.farm_first[cell] as u32;
            s.push_str(&format!(", farmed since year {since}"));
            if on("history") && snap.year.saturating_sub(since) >= 50 {
                s.push_str(", hedged");
            }
        }
        s.push('.');
        lines.push(s);
        if forest > 0.02 && on("frontier") {
            lines.push(format!(
                "Still being cleared: {:.0}% of the trees left.",
                forest * 100.0
            ));
        }
    } else if owner == NONE && snap.built[cell] == NONE && t.is_land(cell) {
        let by_fields = (0..8).any(|k| {
            t.grid
                .neighbor(cell, k)
                .is_some_and(|j| snap.owner[j] != NONE)
        });
        if on("grazing") && by_fields && forest < 0.35 {
            lines.push("Pasture: grazed by the sheep of the village next to it.".into());
        } else if on("grazing") && forest > 0.15 && forest < t.forest0[cell] - 0.08 {
            lines.push("Managed woodland, coppiced for firewood.".into());
        }
        if on("history") && v.farm_last[cell] != NEVER {
            lines.push(format!(
                "Abandoned fields, last farmed in year {}.",
                v.farm_last[cell]
            ));
        }
    }
    if forest > 0.3 && t.is_land(cell) && owner == NONE {
        lines.push(format!("Woodland: {:.0}% tree cover.", forest * 100.0));
    }
    if on("relief") || on("terrain") {
        let mut s = query::describe_cell(t, cell);
        if let Some(c) = s.get_mut(..1) {
            c.make_ascii_uppercase();
        }
        if t.is_land(cell) {
            s.push_str(&format!(
                ", {:.0} m up, yield {:.0}% of the best land",
                t.elevation[cell].max(0.0),
                t.food_yield[cell] * 100.0
            ));
        }
        s.push('.');
        lines.push(s);
    }
    let named: Vec<&str> = t
        .features
        .iter()
        .filter(|f| f.cells.contains(&(cell as u32)))
        .map(|f| f.name.as_str())
        .collect();
    if !named.is_empty() {
        lines.push(named.join(" · "));
    }
    if on("routes") {
        if snap.road[cell] {
            lines.push("On a road.".into());
        }
        for (a, b, f, water) in query::routes_through(snap, t, cell, 2) {
            let by = if water { "by water" } else { "by land" };
            lines.push(format!(
                "Trade: {} → {}, {f:.0} a year {by}.",
                name(a),
                name(b)
            ));
        }
    }
    if on("score") {
        if let Some(sc) = snap.site_score.get(cell).filter(|s| s.is_finite()) {
            lines.push(format!("Site score for a new hamlet: {sc:.2}."));
        }
    }
    for l in lines {
        ui.label(l);
    }
    ui.weak("Click to pin this place in the inspector.");
}
