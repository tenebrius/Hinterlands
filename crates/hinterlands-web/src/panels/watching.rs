//! "What you are watching" (release): a short guide to the phases of a run,
//! open on first load, with the phase the run is in shown in bold. Years are
//! the historical calendar (year 0 = 1100); with events off they are run years.

use super::Panel;
use crate::view::{Msg, ViewState};

/// A phase: first run year, title, and a few plain sentences.
const PHASES: [(u32, &str, &str); 6] = [
    (
        0,
        "Villages fill the farmland (1100–1200)",
        "Ten hamlets clear the best land nearest them and grow. New ones are founded where free farmland, water and a market within reach meet, and many small ones fail.",
    ),
    (
        100,
        "Markets and towns pull ahead",
        "Grain is sold to the markets that draw it most. Towns grow on that trade and on the traffic passing through them. Roads form along the busiest routes and fade where trade falls away; busy crossings get bridges.",
    ),
    (
        150,
        "Works and goods",
        "Salt pans, iron workings and quarries are worked by the place that holds them. Their goods are bought by whoever can reach them, and some works become small specialist towns. Lords plant new towns on the roads from 1250.",
    ),
    (
        215,
        "Famine and pestilence (1315–1349)",
        "The harvests fail for three years from 1315. In 1348 the pestilence takes about two in five people, and it returns in 1361 and 1369. Watch the map darken and the poorer villages empty.",
    ),
    (
        270,
        "Recovery and the planted towns",
        "Survivors leave poor land for the good land the dead have left. The old hierarchy mostly returns. One harbour silts up and loses its trade; some planted towns thrive and others fail.",
    ),
    (
        350,
        "Ships and the coast (1450 onward)",
        "Larger ships make sea travel cheaper. Ships at the map edge stand for the world beyond, which buys grain and sends imports (the wine-red routes) along the coast and up the rivers.",
    ),
];

#[derive(Default)]
pub struct Watching;

impl Panel for Watching {
    fn title(&self) -> &'static str {
        "What you are watching"
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        v: &ViewState,
        _layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg> {
        let year = v.snapshot.year;
        let now = PHASES.iter().rposition(|p| year >= p.0).unwrap_or(0);
        ui.small(
            "A region settles from a few hamlets, 1100 to 1600. Each phase below follows from the one before; nothing is scripted except the famine, the plague, the harbour that silts and the better ships.",
        );
        for (i, (_, title, text)) in PHASES.iter().enumerate() {
            let head = egui::RichText::new(*title).small();
            ui.label(if i == now { head.strong() } else { head.weak() });
            if i == now {
                ui.small(*text);
            }
        }
        Vec::new()
    }
}
