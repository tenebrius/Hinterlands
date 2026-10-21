//! Goods (v2): the year's totals by good, and the site or specialist at the
//! selected cell.

use super::Panel;
use crate::view::{Msg, ViewState};
use hinterlands_core::state::{Good, Kind, NONE};

pub struct GoodsPanel;

fn name(g: Good) -> &'static str {
    match g {
        Good::Salt => "Salt",
        Good::Iron => "Iron",
        Good::Stone => "Stone",
    }
}

impl Panel for GoodsPanel {
    fn title(&self) -> &'static str {
        "Goods"
    }
    fn default_open(&self) -> bool {
        false
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        _layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        let snap = &v.snapshot;
        if snap.sites.is_empty() {
            ui.label("Goods are off.");
            return Vec::new();
        }
        for g in Good::ALL {
            let sites: Vec<_> = snap.sites.iter().filter(|s| s.good == g).collect();
            let working = sites.iter().filter(|s| s.sold > 0.0).count();
            let sold: f32 = sites.iter().map(|s| s.sold).sum();
            ui.label(format!(
                "{}: {working} of {} sites working, {sold:.0} sold this year",
                name(g),
                sites.len()
            ));
        }
        let specialists = snap
            .settlements
            .iter()
            .filter(|s| s.kind != Kind::Agrarian)
            .count();
        let paid: f32 = snap.settlements.iter().map(|s| s.goods_in).sum();
        ui.label(format!(
            "{specialists} specialist settlements; {paid:.0} grain paid for goods"
        ));
        if let Some(cell) = v.selected {
            ui.separator();
            if let Some(site) = snap.sites.iter().find(|s| s.cell as usize == cell) {
                let holder = if site.holder == NONE {
                    "no one".to_string()
                } else {
                    snap.settlement(site.holder)
                        .map_or("no one".into(), |s| s.name.clone())
                };
                ui.label(format!(
                    "{} site, worked by {holder}: capacity {:.0}, sold {:.0}",
                    name(site.good),
                    site.capacity,
                    site.sold
                ));
            }
            if let Some(s) = snap.settlements.iter().find(|s| s.anchor as usize == cell) {
                let kind = match s.kind {
                    Kind::Agrarian => "an ordinary settlement",
                    Kind::Mine => "an iron mine",
                    Kind::Saltern => "salt works",
                    Kind::Quarry => "a quarry",
                };
                let held: Vec<String> = snap
                    .sites
                    .iter()
                    .filter(|x| x.holder == s.id)
                    .map(|x| format!("{} ({:.0} sold)", name(x.good).to_lowercase(), x.sold))
                    .collect();
                ui.label(format!("{} is {kind}.", s.name));
                if !held.is_empty() {
                    ui.label(format!("Works: {}", held.join(", ")));
                }
                if s.goods_in > 0.0 {
                    ui.label(format!("Paid {:.0} grain for goods this year", s.goods_in));
                }
            }
        }
        Vec::new()
    }
}
