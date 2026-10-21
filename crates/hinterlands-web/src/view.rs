//! One `ViewState` holds everything the UI shows; one `Msg` enum is the only way
//! it changes; `update` applies messages. Widgets emit messages, nothing else.

use crate::camera::Camera;
use crate::sim_handle::SimHandle;
use hinterlands_core::metrics::{self, Metrics};
use hinterlands_core::state::LogEntry;
use hinterlands_core::{Params, Snapshot, Terrain};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub enum Msg {
    Step(u32),
    TogglePlay,
    SetSpeed(f32),
    Select(Option<usize>),
    Hover(Option<usize>),
    ToggleLayer(&'static str),
    SetParam(&'static str, f64),
    ResetParams,
    Reset(u64),
    NewMap(u64),
    SetCamera(Camera),
    Save,
    Load(Vec<u8>),
    /// Fix the drawn season (0 spring .. 3 winter), or `None` to follow the clock.
    SetSeason(Option<u8>),
}

pub const SEASONS: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

/// Never farmed, in the farming-history arrays.
pub const NEVER: u16 = u16::MAX;

/// One point of the population chart.
#[derive(Clone, Copy, Debug)]
pub struct YearStats {
    pub year: u32,
    pub total: f32,
    pub largest: f32,
}

pub struct ViewState {
    pub snapshot: Snapshot,
    pub terrain: Terrain,
    pub params: Params,
    pub map_seed: u64,
    pub sim_seed: u64,
    pub metrics: Metrics,
    pub camera: Option<Camera>,
    pub selected: Option<usize>,
    pub hovered: Option<usize>,
    pub layers_on: BTreeSet<&'static str>,
    pub playing: bool,
    /// Simulated years per second.
    pub speed: f32,
    pub history: Vec<YearStats>,
    /// Foundings and failures, newest last.
    pub chronicle: Vec<LogEntry>,
    /// Bumped whenever the terrain changes, so cached layers rebuild.
    pub terrain_version: u64,
    pub status: String,
    /// First and last year each cell was cultivated in this session (NEVER if not).
    pub farm_first: Vec<u16>,
    pub farm_last: Vec<u16>,
    /// Fixed season, or None to follow the clock.
    pub season_fixed: Option<u8>,
    /// Progress through the current year while playing, 0..1 (set by the app).
    pub year_phase: f32,
}

impl ViewState {
    pub fn new(sim: &SimHandle, layers_on: BTreeSet<&'static str>) -> Self {
        let snapshot = sim.snapshot();
        let terrain = sim.terrain().clone();
        let cells = terrain.n();
        let mut v = Self {
            metrics: metrics::compute(&snapshot, &terrain),
            snapshot,
            terrain,
            params: sim.params().clone(),
            map_seed: sim.map_seed,
            sim_seed: sim.sim_seed(),
            camera: None,
            selected: None,
            hovered: None,
            layers_on,
            playing: false,
            speed: 20.0,
            history: Vec::new(),
            chronicle: Vec::new(),
            terrain_version: 0,
            status: String::new(),
            farm_first: vec![NEVER; cells],
            farm_last: vec![NEVER; cells],
            season_fixed: None,
            year_phase: 0.0,
        };
        v.record();
        v
    }

    pub fn layer_on(&self, id: &str) -> bool {
        self.layers_on.contains(id)
    }

    /// The season to draw, 0..1 through the year (spring starts at 0).
    pub fn season_phase(&self, time: f64) -> f32 {
        match self.season_fixed {
            Some(k) => k as f32 * 0.25 + 0.125,
            None if self.playing && self.speed <= 4.0 => self.year_phase,
            None if self.playing => 0.375,
            None => ((time / 8.0) % 1.0) as f32,
        }
    }

    pub fn season_index(&self, time: f64) -> usize {
        ((self.season_phase(time) * 4.0) as usize).min(3)
    }

    fn record(&mut self) {
        let year = self.snapshot.year.min(NEVER as u32 - 1) as u16;
        for (i, &o) in self.snapshot.owner.iter().enumerate() {
            if o != hinterlands_core::state::NONE {
                if self.farm_first[i] == NEVER {
                    self.farm_first[i] = year;
                }
                self.farm_last[i] = year;
            }
        }
        let s = &self.snapshot;
        self.history.push(YearStats {
            year: s.year,
            total: s.total_pop(),
            largest: s.settlements.iter().map(|x| x.pop).fold(0.0, f32::max),
        });
        self.chronicle.extend(s.log.iter().cloned());
        if self.chronicle.len() > 2000 {
            self.chronicle.drain(..self.chronicle.len() - 2000);
        }
    }

    fn refresh(&mut self, sim: &SimHandle) {
        self.snapshot = sim.snapshot();
        self.metrics = metrics::compute(&self.snapshot, &self.terrain);
        self.record();
    }

    fn restart(&mut self, sim: &SimHandle) {
        self.history.clear();
        self.chronicle.clear();
        self.farm_first = vec![NEVER; self.terrain.n()];
        self.farm_last = vec![NEVER; self.terrain.n()];
        self.map_seed = sim.map_seed;
        self.sim_seed = sim.sim_seed();
        self.refresh(sim);
    }
}

pub fn update(v: &mut ViewState, sim: &mut SimHandle, msg: Msg) {
    match msg {
        Msg::Step(n) => {
            for _ in 0..n {
                sim.step(1);
                v.refresh(sim);
            }
        }
        Msg::TogglePlay => v.playing = !v.playing,
        Msg::SetSpeed(s) => v.speed = s.clamp(1.0, 200.0),
        Msg::Select(c) => v.selected = c,
        Msg::Hover(c) => v.hovered = c,
        Msg::ToggleLayer(id) => {
            if !v.layers_on.remove(id) {
                v.layers_on.insert(id);
            }
        }
        Msg::SetParam(name, value) => {
            let mut p = sim.params().clone();
            p.set(name, value);
            sim.set_params(p.clone());
            v.params = p;
        }
        Msg::ResetParams => {
            sim.set_params(Params::default());
            v.params = Params::default();
        }
        Msg::Reset(seed) => {
            sim.reset(seed);
            v.restart(sim);
        }
        Msg::NewMap(seed) => {
            sim.new_map(seed);
            v.terrain = sim.terrain().clone();
            v.terrain_version += 1;
            v.camera = None;
            v.selected = None;
            v.restart(sim);
        }
        Msg::SetCamera(c) => v.camera = Some(c),
        Msg::SetSeason(s) => v.season_fixed = s,
        Msg::Save => {
            let file = crate::app::SaveFile::of(v, sim);
            v.status = crate::app::download(
                "hinterlands-save.json",
                &serde_json::to_string(&file).unwrap_or_default(),
            );
        }
        Msg::Load(bytes) => match serde_json::from_slice::<crate::app::SaveFile>(&bytes) {
            Ok(file) => {
                *sim = SimHandle::new(file.map_seed, file.sim_seed, file.params.clone());
                v.terrain = sim.terrain().clone();
                v.terrain_version += 1;
                v.params = file.params;
                v.camera = None;
                v.restart(sim);
                for _ in 0..file.year {
                    sim.step(1);
                    v.refresh(sim);
                }
                v.status = format!("Loaded year {}", file.year);
            }
            Err(e) => v.status = format!("Could not load: {e}"),
        },
    }
}
