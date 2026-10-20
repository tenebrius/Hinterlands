//! Determinism: the same seeds and params give an identical final snapshot, and
//! that snapshot matches the golden hash committed with the last milestone.

use hinterlands_core::{Params, Sim};
use hinterlands_mapgen::{generate, MapParams};

const GOLDEN: &str = include_str!("golden.txt");

fn final_hash(years: u32) -> u64 {
    let mut sim = Sim::new(generate(1, &MapParams::default()), Params::default(), 1);
    sim.step(years);
    sim.snapshot().hash()
}

#[test]
fn same_seed_same_snapshot() {
    assert_eq!(final_hash(60), final_hash(60));
}

#[test]
fn map_generation_is_deterministic() {
    let a = generate(7, &MapParams::default());
    let b = generate(7, &MapParams::default());
    assert_eq!(a.hash(), b.hash());
    assert_ne!(a.hash(), generate(8, &MapParams::default()).hash());
}

/// Run with `--release`; 500 years. Update `golden.txt` (via `just golden`) only
/// when a change to the model is intended, and say so in the milestone note.
#[test]
fn golden_snapshot_hash() {
    let expected = GOLDEN.trim();
    let got = format!("{:016x}", final_hash(500));
    assert_eq!(
        got, expected,
        "final snapshot hash changed; if intended, run `just golden`"
    );
}
