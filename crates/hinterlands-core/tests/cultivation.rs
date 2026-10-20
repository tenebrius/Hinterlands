//! M2 checks: cultivation and food-only growth, each system alone.

use hinterlands_core::state::State;
use hinterlands_core::{rng_from_seed, step_year, Params, Terrain};

fn food_only() -> Params {
    Params {
        trade_enabled: false,
        founding_enabled: false,
        failure_enabled: false,
        roads_enabled: false,
        ..Params::default()
    }
}

/// One settlement alone on uniform land of the given yield, run for `years`.
fn lone_village(yield_: f32, years: u32) -> (State, Terrain, Params) {
    let mut t = Terrain::flat(41, 41, 1.0);
    t.food_yield.iter_mut().for_each(|y| *y = yield_);
    t.forest0.iter_mut().for_each(|f| *f = 0.8);
    let p = food_only();
    let mut s = State::empty(&t, &p);
    let centre = t.grid.idx(20, 20) as u32;
    s.add_settlement(&t, &p, centre, 50.0, true);
    let mut rng = rng_from_seed(1);
    for _ in 0..years {
        step_year(&mut s, &t, &p, &mut rng);
    }
    (s, t, p)
}

#[test]
fn prime_land_settles_near_150_per_cell_and_stops() {
    let (s, _t, p) = lone_village(1.0, 600);
    let v = &s.settlements[0];
    let per_cell = v.pop / v.fields.len() as f32;
    assert_eq!(
        v.fields.len(),
        v.reach.field_order.len(),
        "all land within r_cult is claimed"
    );
    assert!(
        (per_cell - 150.0).abs() < 10.0,
        "people per prime cell {per_cell}"
    );
    let before = v.history[v.history.len() - 51];
    assert!(
        (v.pop - before).abs() / v.pop < 0.01,
        "stopped growing: {before} -> {}",
        v.pop
    );
    assert!(v.food_ratio > 0.97, "fed at equilibrium: {}", v.food_ratio);
    let _ = p;
}

#[test]
fn poor_land_stays_small() {
    let (rich, ..) = lone_village(1.0, 600);
    let (poor, ..) = lone_village(0.2, 600);
    let (r, q) = (rich.settlements[0].pop, poor.settlements[0].pop);
    assert!(q < 0.3 * r, "poor {q} vs rich {r}");
}

#[test]
fn fields_clear_forest_and_released_cells_regrow() {
    let (mut s, t, p) = lone_village(1.0, 200);
    let field = s.settlements[0].fields[0] as usize;
    assert!(s.forest[field] < 0.01, "cultivated cell cleared");
    // Remove the village: its fields regrow toward natural cover.
    for &c in &s.settlements[0].fields.clone() {
        s.owner[c as usize] = hinterlands_core::state::NONE;
    }
    s.settlements.clear();
    let mut rng = rng_from_seed(1);
    for _ in 0..20 {
        step_year(&mut s, &t, &p, &mut rng);
    }
    assert!(
        (s.forest[field] - 0.4).abs() < 0.02,
        "regrows at 2%/yr: {}",
        s.forest[field]
    );
}

#[test]
fn fields_stretch_along_a_river_bank() {
    // A navigable river along row 20: crossing it costs more than r_cult, so a
    // village beside it farms its own bank only and its fields run along the river.
    let mut t = Terrain::flat(41, 41, 1.0);
    let g = t.grid;
    for x in 0..41 {
        let i = g.idx(x, 20);
        t.water[i] = hinterlands_core::terrain::Water::NavigableRiver;
        t.flow_dir[i] = 4;
    }
    let p = food_only();
    let mut s = State::empty(&t, &p);
    s.add_settlement(&t, &p, g.idx(20, 21) as u32, 50.0, true);
    let mut rng = rng_from_seed(1);
    for _ in 0..400 {
        step_year(&mut s, &t, &p, &mut rng);
    }
    let v = &s.settlements[0];
    assert!(
        v.fields.iter().all(|&c| g.xy(c as usize).1 >= 20),
        "fields stay on the near bank"
    );
    let xs: Vec<i32> = v.fields.iter().map(|&c| g.xy(c as usize).0).collect();
    let ys: Vec<i32> = v.fields.iter().map(|&c| g.xy(c as usize).1).collect();
    let span = |v: &[i32]| v.iter().max().unwrap() - v.iter().min().unwrap();
    assert!(
        span(&xs) > span(&ys),
        "fields stretch along the river: x span {} y span {}",
        span(&xs),
        span(&ys)
    );
}
