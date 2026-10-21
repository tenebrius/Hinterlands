//! One `ViewState` holds everything the UI shows; one `Msg` enum is the only way
//! it changes; `update` applies messages. Widgets emit messages, nothing else.

use crate::camera::Camera;
use crate::sim_handle::SimHandle;
use hinterlands_core::metrics::{self, Metrics};
use hinterlands_core::state::{LogEntry, LogKind, Rank};
use hinterlands_core::{Params, Snapshot, Terrain};
use std::collections::{BTreeMap, BTreeSet};

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

/// A shock on the event timeline (v4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventMark {
    Famine,
    Plague,
    Silted,
    Planted,
}

/// What a chronicle note is about, for the chronicle's filters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoteKind {
    Growth,
    Lead,
    Roads,
}

/// A milestone the viewer notices between years: a place reaching a new size,
/// a new largest place, new roads. Kept beside the core's own log entries.
#[derive(Clone, Debug)]
pub struct Note {
    pub year: u32,
    pub text: String,
    pub anchor: u32,
    pub kind: NoteKind,
}

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
    /// Shocks for the timeline (v4): year and kind, oldest first, one per
    /// event (a plague once, in the year it strikes).
    pub events: Vec<(u32, EventMark)>,
    /// Milestones the viewer noticed, newest last.
    pub notes: Vec<Note>,
    /// The largest rank each settlement has reached, so growth is noted once.
    pub best_rank: BTreeMap<u32, Rank>,
    /// The largest place, and the length of road built, as last noted.
    pub leader: Option<u32>,
    pub road_km: f32,
    /// Bumped whenever the terrain changes, so cached layers rebuild.
    pub terrain_version: u64,
    /// The cell each cell is drawn on when routes are drawn: a sea cell within
    /// `LANE_KM` of land is drawn on the nearest sea cell next to the coast, so
    /// parallel sea paths along a coast bundle into one lane. Drawing only.
    pub sea_lane: Vec<u32>,
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
            events: Vec::new(),
            notes: Vec::new(),
            best_rank: BTreeMap::new(),
            leader: None,
            road_km: 0.0,
            terrain_version: 0,
            sea_lane: Vec::new(),
            status: String::new(),
            farm_first: vec![NEVER; cells],
            farm_last: vec![NEVER; cells],
            season_fixed: None,
            year_phase: 0.0,
        };
        v.sea_lane = sea_lanes(&v.terrain);
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
        // A plague logs every place it strikes; the chronicle keeps the places
        // that lost at least 300, and the timeline one mark.
        for e in &s.log {
            let mark = match e.kind {
                LogKind::Famine { .. } => Some(EventMark::Famine),
                LogKind::Plague { .. } => Some(EventMark::Plague),
                LogKind::Silted => Some(EventMark::Silted),
                LogKind::Planted => Some(EventMark::Planted),
                _ => None,
            };
            if let Some(m) = mark {
                if !self.events.contains(&(e.year, m)) {
                    self.events.push((e.year, m));
                }
            }
        }
        self.chronicle.extend(
            s.log
                .iter()
                .filter(|e| !matches!(e.kind, LogKind::Plague { lost } if lost < 300.0))
                .cloned(),
        );
        if self.chronicle.len() > 2000 {
            self.chronicle.drain(..self.chronicle.len() - 2000);
        }
        self.note_milestones();
    }

    /// Notes what changed this year that the core's log does not record. The
    /// first year only sets the baseline.
    fn note_milestones(&mut self) {
        let s = &self.snapshot;
        let first = self.history.len() <= 1;
        let mut notes = Vec::new();
        for x in &s.settlements {
            let best = self.best_rank.entry(x.id).or_insert(x.rank);
            if x.rank > *best {
                *best = x.rank;
                if !first {
                    let what = match x.rank {
                        Rank::Hamlet => continue,
                        Rank::Village => "a village",
                        Rank::Town => "a town",
                        Rank::City => "a city",
                    };
                    notes.push(Note {
                        year: s.year,
                        text: format!("{} grows into {what} ({:.0} people).", x.name, x.pop),
                        anchor: x.anchor,
                        kind: NoteKind::Growth,
                    });
                }
            }
        }
        // A new largest place, once it leads by 2%, so near-ties do not flicker.
        if let Some(top) = s.settlements.iter().max_by(|a, b| a.pop.total_cmp(&b.pop)) {
            let old = self.leader.and_then(|id| s.settlement(id));
            let takes_lead = match old {
                Some(o) => o.id != top.id && top.pop > o.pop * 1.02,
                None => true,
            };
            if takes_lead {
                if !first {
                    let text = match old {
                        Some(o) => format!(
                            "{} overtakes {} as the largest place ({:.0} people).",
                            top.name, o.name, top.pop
                        ),
                        None => format!(
                            "{} is now the largest place ({:.0} people).",
                            top.name, top.pop
                        ),
                    };
                    notes.push(Note {
                        year: s.year,
                        text,
                        anchor: top.anchor,
                        kind: NoteKind::Lead,
                    });
                }
                self.leader = Some(top.id);
            }
        }
        let g = self.terrain.grid;
        let km: f32 = s
            .road_edges
            .iter()
            .map(|&(a, b)| g.dist_km(a as usize, b as usize))
            .sum();
        if km > self.road_km + 0.5 && !first {
            let anchor = s.road_edges.first().map_or(0, |e| e.0);
            notes.push(Note {
                year: s.year,
                text: format!(
                    "{:.0} km of new road are built ({:.0} km in all).",
                    km - self.road_km,
                    km
                ),
                anchor,
                kind: NoteKind::Roads,
            });
        }
        self.road_km = km;
        self.notes.extend(notes);
        if self.notes.len() > 2000 {
            self.notes.drain(..self.notes.len() - 2000);
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
        self.events.clear();
        self.notes.clear();
        self.best_rank.clear();
        self.leader = None;
        self.road_km = 0.0;
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
            v.sea_lane = sea_lanes(&v.terrain);
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
                v.sea_lane = sea_lanes(&v.terrain);
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

/// Sea cells within this distance of land are drawn on the coastal lane.
const LANE_KM: f32 = 4.0;

/// For each cell, the cell its routes are drawn on (see `ViewState::sea_lane`).
pub fn sea_lanes(t: &Terrain) -> Vec<u32> {
    let g = t.grid;
    let coastal_sea =
        |i: usize| t.is_sea(i) && (0..8).any(|d| g.neighbor(i, d).is_some_and(|j| t.is_land(j)));
    (0..t.n())
        .map(|i| {
            if !t.is_sea(i) || coastal_sea(i) {
                return i as u32;
            }
            g.disk(i, LANE_KM)
                .filter(|&j| coastal_sea(j))
                .min_by(|&a, &b| g.dist_km(i, a).total_cmp(&g.dist_km(i, b)).then(a.cmp(&b)))
                .map_or(i as u32, |j| j as u32)
        })
        .collect()
}
