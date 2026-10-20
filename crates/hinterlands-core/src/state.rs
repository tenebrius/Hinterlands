//! Everything the yearly tick changes. Struct-of-arrays per cell, plus the
//! settlement list kept sorted by id.

use crate::cost::{CostGrid, Dijkstra, Transport};
use crate::event::Event;
use crate::params::Params;
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};

pub const NONE: u32 = u32::MAX;
/// Quantisation of stored travel costs: hundredths of a km-equivalent.
pub const COST_SCALE: f32 = 100.0;
pub const COST_UNREACHED: u16 = u16::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Agrarian,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Hamlet,
    Village,
    Town,
    City,
}

impl Rank {
    pub fn of(pop: f32) -> Self {
        if pop < 100.0 {
            Rank::Hamlet
        } else if pop < 500.0 {
            Rank::Village
        } else if pop < 2000.0 {
            Rank::Town
        } else {
            Rank::City
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Rank::Hamlet => "hamlet",
            Rank::Village => "village",
            Rank::Town => "town",
            Rank::City => "city",
        }
    }
}

/// Another settlement within trading reach, with the stored least-cost path to it.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Partner {
    pub id: u32,
    pub cost: f32,
    /// Graph nodes from this settlement's anchor to the partner's, inclusive.
    pub path: Vec<u32>,
}

/// What a settlement can reach, refreshed every `cost_refresh_years`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Reach {
    /// Travel cost from the anchor to each cell (either layer), quantised.
    pub cost_field: Vec<u16>,
    /// Land cells within `r_cult` by walking cost, nearest first.
    pub field_order: Vec<u32>,
    /// Land cells within `r_fuel` by walking cost, nearest first.
    pub fuel_order: Vec<u32>,
    /// Settlements within `c_max`, by id.
    pub partners: Vec<Partner>,
    pub refreshed: u32,
}

impl Reach {
    #[inline]
    pub fn cost_to_cell(&self, cell: usize) -> f32 {
        let q = self.cost_field[cell];
        if q == COST_UNREACHED {
            f32::INFINITY
        } else {
            q as f32 / COST_SCALE
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settlement {
    pub id: u32,
    pub anchor: u32,
    pub pop: f32,
    pub founded: u32,
    /// True for the starting hamlets; failure statistics count only the rest.
    pub initial: bool,
    pub kind: Kind,
    /// Last year food covered need.
    pub last_fed: u32,
    /// Consecutive years below the failing population.
    pub low_years: u32,
    /// Food-driven loss, smoothed over `loss_years`.
    pub smoothed_loss: f32,
    /// Cultivated cells in the order they were claimed.
    pub fields: Vec<u32>,
    // Yearly accounts, written by the systems that own them.
    pub food_prod: f32,
    pub resource_out: f32,
    pub food_in: f32,
    pub food_out: f32,
    pub trade_in: f32,
    pub through: f32,
    pub food_ratio: f32,
    /// Population over the last 51 years, oldest first.
    pub history: Vec<f32>,
    pub reach: Reach,
}

impl Settlement {
    pub fn new(id: u32, anchor: u32, pop: f32, year: u32, initial: bool) -> Self {
        Self {
            id,
            anchor,
            pop,
            founded: year,
            initial,
            kind: Kind::Agrarian,
            last_fed: year,
            low_years: 0,
            smoothed_loss: 0.0,
            fields: Vec::new(),
            food_prod: 0.0,
            resource_out: 0.0,
            food_in: 0.0,
            food_out: 0.0,
            trade_in: 0.0,
            through: 0.0,
            food_ratio: 1.0,
            history: Vec::new(),
            reach: Reach::default(),
        }
    }

    pub fn need(&self, p: &Params) -> f32 {
        self.pop * p.food_per_person
    }

    pub fn rank(&self) -> Rank {
        Rank::of(self.pop)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flow {
    pub from: u32,
    pub to: u32,
    /// Trade value per year, `T_ij`.
    pub amount: f32,
    /// Food actually shipped along the same path this year.
    pub food: f32,
    /// Index of the path in the exporter's partner list.
    pub partner_idx: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum FailCause {
    Depopulated,
    Starved,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LogKind {
    Initial,
    Founded {
        score: f32,
        donor: u32,
    },
    Failed {
        cause: FailCause,
        moved_to: Option<u32>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub year: u32,
    pub settlement: u32,
    pub anchor: u32,
    pub pop: f32,
    pub kind: LogKind,
}

/// Running totals for validation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tally {
    pub founded: u32,
    /// Settlements founded during the run that later failed.
    pub failed: u32,
    /// Best site score at the last founding step, founded or not.
    pub last_best_score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub year: u32,
    pub forest: Vec<f32>,
    /// Owning settlement id of a cultivated cell, or `NONE`.
    pub owner: Vec<u32>,
    pub road: Vec<bool>,
    /// Political domain per cell; unused in v1.
    pub domain: Vec<u16>,
    pub transport: Transport,
    pub cost: CostGrid,
    /// Sorted by id.
    pub settlements: Vec<Settlement>,
    pub next_id: u32,
    pub flows: Vec<Flow>,
    /// Trade carried through each cell this year (both layers summed).
    pub cell_load: Vec<f32>,
    /// Route junctions: cells where routes meet from three or more directions.
    pub hub: Vec<bool>,
    /// Site score at the last founding step (NaN where not a candidate).
    pub site_score: Vec<f32>,
    pub log: Vec<LogEntry>,
    pub tally: Tally,
    /// Event queue applied at the start of each year; empty in v1.
    pub events: Vec<Event>,
    #[serde(skip)]
    pub scratch: Dijkstra,
}

impl State {
    /// A world with no settlements yet; `init::populate` adds the starting hamlets.
    pub fn empty(t: &Terrain, p: &Params) -> Self {
        let n = t.n();
        let road = vec![false; n];
        let transport = Transport::from_params(p);
        let cost = CostGrid::build(t, &road, &transport);
        Self {
            year: 0,
            forest: t.forest0.clone(),
            owner: vec![NONE; n],
            road,
            domain: vec![0; n],
            transport,
            cost,
            settlements: Vec::new(),
            next_id: 0,
            flows: Vec::new(),
            cell_load: vec![0.0; n],
            hub: vec![false; n],
            site_score: vec![f32::NAN; n],
            log: Vec::new(),
            tally: Tally::default(),
            events: Vec::new(),
            scratch: Dijkstra::new(n),
        }
    }

    pub fn index_of(&self, id: u32) -> Option<usize> {
        self.settlements.binary_search_by_key(&id, |s| s.id).ok()
    }

    pub fn get(&self, id: u32) -> Option<&Settlement> {
        self.index_of(id).map(|i| &self.settlements[i])
    }

    pub fn total_pop(&self) -> f32 {
        self.settlements.iter().map(|s| s.pop).sum()
    }

    pub fn rebuild_costs(&mut self, t: &Terrain) {
        self.cost = CostGrid::build(t, &self.road, &self.transport);
    }

    /// Add a settlement and compute its reach immediately.
    pub fn add_settlement(
        &mut self,
        t: &Terrain,
        p: &Params,
        anchor: u32,
        pop: f32,
        initial: bool,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let mut s = Settlement::new(id, anchor, pop, self.year, initial);
        s.history.push(pop);
        self.settlements.push(s);
        let idx = self.settlements.len() - 1;
        crate::reach::refresh_one(self, t, p, idx);
        id
    }
}
