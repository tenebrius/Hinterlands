//! Hinterlands simulation core: a pure library with no I/O. Params and seeds go
//! in; `step_year` advances the `State`; a `Snapshot` is the read-only view.

pub mod cost;
pub mod event;
pub mod footprint;
pub mod grid;
pub mod init;
pub mod metrics;
pub mod names;
pub mod params;
pub mod query;
pub mod reach;
pub mod snapshot;
pub mod state;
pub mod systems;
pub mod terrain;

pub use params::Params;
pub use snapshot::Snapshot;
pub use state::State;
pub use terrain::Terrain;

use rand::SeedableRng;

pub type Rng = rand_chacha::ChaCha8Rng;

pub fn rng_from_seed(seed: u64) -> Rng {
    Rng::seed_from_u64(seed)
}

/// FNV-1a, 64-bit. Stable across platforms; used for snapshot and terrain hashes.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Advance one year: events, then the six systems in the fixed order.
pub fn step_year(state: &mut State, t: &Terrain, p: &Params, rng: &mut Rng) {
    state.log.clear();
    state.money = Default::default();
    systems::events::run(state, t, p, rng);
    let events = std::mem::take(&mut state.events);
    for e in &events {
        e.apply(state, t, p);
    }
    reach::refresh_due(state, t, p);
    systems::harvest::run(state, t, p, rng);
    systems::cultivation::run(state, t, p, rng);
    systems::production::run(state, t, p, rng);
    systems::trade::run(state, t, p, rng);
    systems::market::run(state, t, p, rng);
    systems::growth::run(state, t, p, rng);
    systems::founding::run(state, t, p, rng);
    systems::failure::run(state, t, p, rng);
    systems::roads::run(state, t, p, rng);
    footprint::update(state, t, p);
    state.year += 1;
}

/// The simulation and everything needed to advance it: the only door the viewer
/// and the CLI use.
#[derive(Clone)]
pub struct Sim {
    pub terrain: Terrain,
    pub terrain_hash: u64,
    pub state: State,
    pub params: Params,
    pub sim_seed: u64,
    pub rng: Rng,
}

impl Sim {
    pub fn new(terrain: Terrain, params: Params, sim_seed: u64) -> Self {
        let terrain_hash = terrain.hash();
        let mut state = State::empty(&terrain, &params);
        init::populate(&mut state, &terrain, &params);
        let mut rng = rng_from_seed(sim_seed);
        // The weather is drawn first, so a seed keeps it whatever else draws (D75).
        systems::harvest::init(&mut state, &terrain, &params, &mut rng);
        Self {
            terrain,
            terrain_hash,
            state,
            params,
            sim_seed,
            rng,
        }
    }

    pub fn step(&mut self, years: u32) {
        for _ in 0..years {
            step_year(&mut self.state, &self.terrain, &self.params, &mut self.rng);
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot::of(&self.state, &self.terrain, self.terrain_hash, &self.params)
    }

    pub fn reset(&mut self, sim_seed: u64) {
        *self = Sim::new(self.terrain.clone(), self.params.clone(), sim_seed);
    }

    /// Change parameters mid-run. Transport ratios are copied into the state and
    /// the costs rebuilt, as an event would.
    pub fn set_params(&mut self, params: Params) {
        let transport = cost::Transport::from_params(&params);
        self.params = params;
        if transport != self.state.transport {
            self.state.transport = transport;
            self.state.rebuild_costs(&self.terrain);
            reach::refresh_all(&mut self.state, &self.terrain, &self.params);
        }
    }
}
