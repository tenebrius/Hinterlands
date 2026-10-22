//! "What you are watching" (release): a short guide to the phases of a run,
//! open on first load, with the phase the run is in shown in bold. Years are
//! the historical calendar (year 0 = 1100); with events off they are run years.

use super::Panel;
use crate::view::{Msg, ViewState};

/// A phase: first run year, title, and a few plain sentences.
const PHASES: [(u32, &str, &str); 7] = [
    (
        0,
        "The land fills (1100–1200)",
        "Ten hamlets clear the best land nearest them and grow. New ones are founded where free farmland, water and a market within reach meet, and many small ones fail.",
    ),
    (
        100,
        "Markets and the grain trade",
        "Rents and surplus grain go to market and sell at a price. Market towns grow on the grain they draw from their hinterlands; their prices follow the region's, dearer where harvests are poor. Roads form along the busiest routes and busy crossings get bridges.",
    ),
    (
        150,
        "Wool and coin",
        "Land not worth ploughing goes to sheep, outward and on poor soil. Wool goes down to the ports (cream routes) and coin comes back up; farmers and lords keep purses, and the region's prices settle where its coin and its grain meet. Market stores buy in cheap years.",
    ),
    (
        215,
        "Famine and murrain (1315–22)",
        "The harvests fail from 1315, worst in 1316. Grain prices leap, purses empty, and the stores are drawn down. Scab and murrain thin the flocks into the 1320s.",
    ),
    (
        248,
        "Pestilence and the long depopulation (1348–1480)",
        "The pestilence of 1348 takes about two in five people. It returns again and again, and disease never quite leaves: a century later the region holds about half its people of 1340.",
    ),
    (
        280,
        "The turn to grass",
        "With fewer hands, released fields go down to grass, on the poorest and farthest land first. Wool brings in nearly all the region's coin; poor-land villages empty, and the chronicle records their fields put down to grass.",
    ),
    (
        380,
        "Ships and the coast (1480 onward)",
        "The plague years end and the region grows again. Larger ships make sea travel cheaper; ships at the map edge stand for the world beyond, which buys wool and grain and sends imports (the wine-red routes) along the coast and up the rivers.",
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
            "A region settles from a few hamlets, 1100 to 1600. Each phase below follows from the one before; nothing is scripted except the calendar of harvest failures, plague, murrain and better ships, and the wool price set abroad.",
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
