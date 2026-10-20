//! The only door to the simulation. Message-shaped so it can move to a web worker.

use hinterlands_core::{Params, Sim, Snapshot, Terrain};
use hinterlands_mapgen::{generate, MapParams};

pub struct SimHandle {
    sim: Sim,
    pub map_seed: u64,
}

impl SimHandle {
    pub fn new(map_seed: u64, sim_seed: u64, params: Params) -> Self {
        let terrain = generate(map_seed, &MapParams::default());
        Self {
            sim: Sim::new(terrain, params, sim_seed),
            map_seed,
        }
    }

    pub fn step(&mut self, years: u32) {
        self.sim.step(years);
    }

    pub fn snapshot(&self) -> Snapshot {
        self.sim.snapshot()
    }

    pub fn terrain(&self) -> &Terrain {
        &self.sim.terrain
    }

    pub fn params(&self) -> &Params {
        &self.sim.params
    }

    pub fn set_params(&mut self, p: Params) {
        self.sim.set_params(p);
    }

    pub fn sim_seed(&self) -> u64 {
        self.sim.sim_seed
    }

    /// Start again with a new simulation seed on the same terrain.
    pub fn reset(&mut self, sim_seed: u64) {
        self.sim.reset(sim_seed);
    }

    /// New terrain from a new map seed, keeping params and the simulation seed.
    pub fn new_map(&mut self, map_seed: u64) {
        *self = Self::new(map_seed, self.sim.sim_seed, self.sim.params.clone());
    }
}
