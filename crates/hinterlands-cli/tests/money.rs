//! D96, D97: coin moves only by payment. Each year the stock in all purses
//! changes by exactly the ports' grain payments and planted towns' coin, less
//! imports, luxuries bought abroad, wear, and the purses of places removed
//! with no partner; with wool and cloth, plus what the ports paid for them
//! (D2).

use hinterlands_core::{Params, Sim};
use hinterlands_mapgen::{generate, MapParams};

#[test]
fn coin_is_conserved() {
    conserved(Params::default());
}

/// With the ports in reach of every place and cloth (D1b, D2): export cloth
/// coin in.
#[test]
fn coin_is_conserved_with_cloth() {
    let p = Params {
        port_reach: true,
        cloth: true,
        ..Params::default()
    };
    conserved(p);
}

fn conserved(params: Params) {
    let mut sim = Sim::new(generate(1, &MapParams::default()), params, 1);
    let mut moved = 0f32;
    for _ in 0..150 {
        let before = sim.state.money_stock();
        sim.step(1);
        let l = sim.state.money;
        let after = sim.state.money_stock();
        let expect = before + l.port_in + l.wool_in + l.planted_in + l.cloth_in
            - l.imports_out
            - l.luxury_out
            - l.wear
            - l.removed;
        assert!(
            (after - expect).abs() <= 1e-4 * after.max(1.0),
            "year {}: stock {after} against {expect}",
            sim.state.year
        );
        moved += l.port_in + l.imports_out;
        for s in &sim.state.settlements {
            assert!(
                s.purse >= -1e-3 && s.mpurse >= -1e-3,
                "negative purse at {}",
                s.id
            );
        }
    }
    assert!(moved > 0.0, "no coin crossed the border");
}
