//! W1 checks (D75): a seed's weather does not depend on what else the run draws,
//! and the multiplier has the configured spread at the cells.

use hinterlands_core::{Params, Sim, Terrain};

fn sim(events_mode: u32) -> Sim {
    let mut t = Terrain::flat(64, 64, 1.0);
    t.food_yield.iter_mut().for_each(|y| *y = 1.0);
    let p = Params {
        local_harvests: true,
        harvest_years: 20,
        events_mode,
        ..Params::default()
    };
    Sim::new(t, p, 3)
}

#[test]
fn weather_is_the_same_with_events_on_or_off() {
    let (mut a, mut b) = (sim(1), sim(0));
    // Past the pre-drawn years too: later years are drawn when reached.
    for _ in 0..25 {
        a.step(1);
        b.step(1);
    }
    assert!(!a.state.harvest_nodes.is_empty());
    assert_eq!(a.state.harvest_nodes, b.state.harvest_nodes);
    assert_eq!(a.state.harvest, b.state.harvest);
}

#[test]
fn multiplier_spread_at_cells() {
    let mut s = sim(0);
    let (mut n, mut sum, mut sq) = (0f64, 0f64, 0f64);
    for _ in 0..20 {
        s.step(1);
        for &m in &s.state.harvest {
            let d = m as f64 - 1.0;
            n += 1.0;
            sum += d;
            sq += d * d;
        }
    }
    let sd = (sq / n - (sum / n).powi(2)).sqrt();
    assert!((0.13..0.17).contains(&sd), "sd {sd}");
}
