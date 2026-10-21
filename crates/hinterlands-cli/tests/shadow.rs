//! D77: the shadow market (S0) is read by nothing, so turning it on leaves the
//! run unchanged apart from the prices the snapshot carries.

use hinterlands_core::{Params, Sim};
use hinterlands_mapgen::{generate, MapParams};

#[test]
fn shadow_does_not_change_the_run() {
    let run = |shadow: bool| {
        let p = Params {
            market_shadow: shadow,
            ..Params::default()
        };
        let mut sim = Sim::new(generate(1, &MapParams::default()), p, 1);
        sim.step(120);
        let mut snap = sim.snapshot();
        for s in snap.settlements.iter_mut() {
            s.price = None;
        }
        snap.hash()
    };
    assert_eq!(run(false), run(true));
}
