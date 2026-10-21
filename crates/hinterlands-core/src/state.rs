//! Everything the yearly tick changes. Struct-of-arrays per cell, plus the
//! settlement list kept sorted by id.

use crate::cost::{CostGrid, Dijkstra, Transport};
use crate::event::Event;
use crate::params::Params;
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};

pub const NONE: u32 = u32::MAX;
/// Partner id of outside port 0 (v5, D64); port k is `OUTSIDE - k`.
pub const OUTSIDE: u32 = u32::MAX - 1;
/// Most outside ports a map can have (one per side).
pub const MAX_PORTS: u32 = 4;

/// The outside port a partner id names, if it is one.
pub fn port_of(id: u32) -> Option<usize> {
    (id <= OUTSIDE && id > OUTSIDE - MAX_PORTS).then(|| (OUTSIDE - id) as usize)
}
/// Quantisation of stored travel costs: hundredths of a km-equivalent.
pub const COST_SCALE: f32 = 100.0;
pub const COST_UNREACHED: u16 = u16::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Agrarian,
    /// Specialists at a resource site (v2, D40).
    Mine,
    Saltern,
    Quarry,
}

/// The traded goods besides grain (v2, D38).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Good {
    Salt,
    Iron,
    Stone,
}

impl Good {
    pub const ALL: [Good; 3] = [Good::Salt, Good::Iron, Good::Stone];

    pub fn of(r: crate::terrain::Resource) -> Option<Good> {
        match r {
            crate::terrain::Resource::Salt => Some(Good::Salt),
            crate::terrain::Resource::Ore => Some(Good::Iron),
            crate::terrain::Resource::Stone => Some(Good::Stone),
            crate::terrain::Resource::None => None,
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }
}

/// A resource site and what it produced this year (v2, D38). Production writes
/// holder, potential, output (capacity) and fuel_want; cultivation writes
/// fuel_got; trade writes sold.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    pub cell: u32,
    pub good: Good,
    /// The settlement working the site, or `NONE`.
    pub holder: u32,
    /// Output the site's workers could reach this year, grain units.
    pub potential: f32,
    /// Capacity this year, grain units (the iron fuel cap applied).
    pub output: f32,
    /// What the site sold this year, its holder's own use included (written by
    /// trade, D41): a site's output is what it sells.
    pub sold: f32,
    /// Fuel the site asks of the holder's woods for next year (iron only).
    pub fuel_want: f32,
    /// Fuel cut for the site this year.
    pub fuel_got: f32,
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
    pub name: String,
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
    /// Gross market inflow: food delivered to this market plus the share of its
    /// own harvest sold at home (Rihll–Wilson's D_j).
    pub market_in: f32,
    /// Market size W_j (D26), in people: the traders and craftsmen the market
    /// supports. Its own state, as in Rihll–Wilson, used when `market_state` is on.
    pub market: f32,
    /// Trade changing mode (boat to land, river to sea) within a cell of the
    /// anchor this year: what a port handles (D27).
    pub transship: f32,
    /// Grain received this year as payment for goods sold (v2; written by trade).
    pub goods_in: f32,
    /// A stream or river with enough drainage among the fields (D56; cultivation).
    pub mill: bool,
    /// Planted by a lord (D63).
    pub planted: bool,
    /// Market prices (v6): grain, salt, iron, stone, imports; the goods by
    /// `market_goods` (S1), grain by `market_grain` (S2); zero otherwise.
    pub price: [f32; 5],
    /// Shadow prices, same order (v6 S0, D69), written only by the shadow
    /// solve and read by nothing in the model (D77).
    #[serde(default)]
    pub shadow_price: [f32; 5],
    /// Supply prices of salt, iron and stone where the place holds sites (D71).
    pub supply_price: [f32; 3],
    /// Ten-year averages of goods sales and gross inflow (v2, D44; growth).
    pub avg_goods_in: f32,
    pub avg_inflow: f32,
    pub food_ratio: f32,
    /// Population over the last 51 years, oldest first.
    pub history: Vec<f32>,
    pub reach: Reach,
}

impl Settlement {
    pub fn new(id: u32, name: String, anchor: u32, pop: f32, year: u32, initial: bool) -> Self {
        Self {
            id,
            name,
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
            market_in: 0.0,
            market: 0.0,
            transship: 0.0,
            goods_in: 0.0,
            mill: false,
            planted: false,
            price: [0.0; 5],
            shadow_price: [0.0; 5],
            supply_price: [0.0; 3],
            avg_goods_in: 0.0,
            avg_inflow: 0.0,
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
    /// The good carried, for goods flows (v2); `None` for grain.
    pub good: Option<Good>,
    /// Grain paid for goods (v2), travelling from the buyer to the producer.
    pub payment: bool,
    /// Imports from an outside port (v5, D64), carried on the buyer's path.
    pub import: bool,
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
    /// A specialist settlement founded at a resource site (v2, D40).
    Works {
        good: Good,
        demand: f32,
        donor: u32,
    },
    /// A specialist whose goods sales fell below `specialist_share` of its
    /// inflow became an ordinary settlement (v2, D44).
    Outgrew {
        good: Good,
    },
    /// A famine began (v4, D57); not tied to a settlement.
    Famine {
        years: u32,
    },
    /// Plague struck the settlement; `lost` is the people it will lose (D57).
    Plague {
        lost: f32,
    },
    /// The settlement's harbour silted up (D60).
    Silted,
    /// A lord planted the settlement (D63).
    Planted,
    /// Larger ships now sail the coast (v5, D65); not tied to a settlement.
    Ships,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub year: u32,
    pub settlement: u32,
    pub name: String,
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
    /// Cell pairs a road runs between, sorted; for drawing only (costs use `road`).
    pub road_edges: Vec<(u32, u32)>,
    /// With `traffic_roads` (D54): each cell's road condition, 0 where there is
    /// no road; empty otherwise.
    pub road_q: Vec<f32>,
    /// With `traffic_bridges` (D55): bridges the model has built, and the road
    /// steps in a row each river crossing has carried `bridge_load`; empty
    /// otherwise.
    pub bridge: Vec<bool>,
    pub bridge_streak: Vec<u8>,
    /// Extra cost of each land–water step at silted harbour cells (D60); empty
    /// until a harbour silts. `silt_year` is the drawn year of the historical
    /// silting, 0 before it is drawn.
    pub silt: Vec<f32>,
    pub silt_year: u32,
    /// Year the last plague struck, or `NONE` (D61).
    pub last_plague: u32,
    /// Anchors of the harbours that have silted, in order (D60; for the view).
    pub silted_ports: Vec<u32>,
    /// Shadow market (D69): the outside ports' grain prices, the year's
    /// unmatched excess demand per good as a share of demand, and the grain
    /// arcs carrying at least one unit (from, to, flow, price gap, transport).
    #[serde(default)]
    pub port_price: Vec<f32>,
    #[serde(default)]
    pub market_residual: [f32; 5],
    #[serde(default)]
    pub market_arcs: Vec<(u32, u32, f32, f32, f32)>,
    /// The shadow solve's grain arcs, as `market_arcs` (D77).
    #[serde(default)]
    pub shadow_arcs: Vec<(u32, u32, f32, f32, f32)>,
    /// Goods demand left unmet this year by the equilibrium (D71), by good.
    #[serde(default)]
    pub market_unmet: [f32; 5],
    /// Settlement whose built-up footprint covers the cell, or `NONE` (rendering only).
    pub built: Vec<u32>,
    /// Years a ruin remains visible.
    pub ruin: Vec<u8>,
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
    /// Event queue applied at the start of each year (D57).
    pub events: Vec<Event>,
    /// A famine lasts while `year < famine_until`, taking `famine_loss` of yield.
    pub famine_until: u32,
    pub famine_loss: f32,
    /// This year's yield multiplier per cell (v6 W1); empty unless `local_harvests`.
    #[serde(default)]
    pub harvest: Vec<f32>,
    /// The weather's node values, year after year (D75); empty unless `local_harvests`.
    #[serde(default)]
    pub harvest_nodes: Vec<f32>,
    /// Outside ports as water-layer nodes (v5, D64); empty unless `outside_ports`.
    #[serde(default)]
    pub outside: Vec<u32>,
    /// This year's grain bought by the outside ports and imports sold (D64).
    #[serde(default)]
    pub outside_trade: (f32, f32),
    /// Resource sites, in cell order (v2).
    #[serde(default)]
    pub sites: Vec<Site>,
    #[serde(skip)]
    pub scratch: Dijkstra,
    /// Founding's cache of free food near each cell (NaN = stale), with the
    /// owners, roads and costs it was computed under.
    #[serde(skip)]
    pub food_cache: FoodCache,
}

#[derive(Clone, Debug, Default)]
pub struct FoodCache {
    pub value: Vec<f32>,
    pub owner: Vec<u32>,
    pub road: Vec<bool>,
    pub transport: Option<Transport>,
    pub key: (u32, u32),
    /// Least walking cost per km in the graph it was computed on.
    pub min_per_km: f32,
}

/// Outside ports (v5, D64) as water-layer nodes: the terrain's own when it
/// has them (imported maps), else one per open-sea map edge (a side with at
/// least three sea cells), at the edge sea cell nearest the busiest coast. The
/// busiest coast is taken as the harbour cell with the most yield within 10 km:
/// the coast that can carry the most people. Ties go to the lower cell.
fn outside_ports(t: &Terrain) -> Vec<u32> {
    let g = t.grid;
    let n = t.n() as u32;
    if !t.outside_ports.is_empty() {
        return t.outside_ports.iter().map(|&c| n + c).collect();
    }
    let busiest = (0..t.n())
        .filter(|&i| t.harbour[i])
        .map(|i| {
            let y: f32 = g
                .disk(i, 10.0)
                .filter(|&j| t.is_land(j))
                .map(|j| t.food_yield[j])
                .sum();
            (y, i)
        })
        .max_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)))
        .map(|(_, i)| i);
    let Some(coast) = busiest else {
        return Vec::new();
    };
    let (w, h) = (g.w as i32, g.h as i32);
    let sides: [fn(i32, i32, i32, i32) -> bool; 4] = [
        |_, y, _, _| y == 0,
        |x, _, w, _| x == w - 1,
        |_, y, _, h| y == h - 1,
        |x, _, _, _| x == 0,
    ];
    let mut out = Vec::new();
    for side in sides {
        let cells: Vec<usize> = (0..t.n())
            .filter(|&i| {
                let (x, y) = g.xy(i);
                side(x, y, w, h) && t.is_sea(i)
            })
            .collect();
        if cells.len() < 3 {
            continue;
        }
        let best = cells.iter().copied().min_by(|&a, &b| {
            g.dist_km(a, coast)
                .total_cmp(&g.dist_km(b, coast))
                .then(a.cmp(&b))
        });
        if let Some(c) = best {
            let node = n + c as u32;
            if !out.contains(&node) {
                out.push(node);
            }
        }
    }
    out
}

impl State {
    /// A world with no settlements yet; `init::populate` adds the starting hamlets.
    pub fn empty(t: &Terrain, p: &Params) -> Self {
        let n = t.n();
        // Roads given as input (D32) are there from the first year.
        let road: Vec<bool> = if t.initial_road.len() == n {
            t.initial_road.clone()
        } else {
            vec![false; n]
        };
        let transport = Transport::from_params(p);
        let cost = CostGrid::build(t, &road, &transport);
        // With traffic roads (D54), initial roads start at full condition.
        let road_q = if p.traffic_roads {
            road.iter().map(|&r| if r { 1.0 } else { 0.0 }).collect()
        } else {
            Vec::new()
        };
        Self {
            year: 0,
            forest: t.forest0.clone(),
            owner: vec![NONE; n],
            road,
            road_edges: Vec::new(),
            road_q,
            bridge: if p.traffic_bridges {
                vec![false; n]
            } else {
                Vec::new()
            },
            bridge_streak: if p.traffic_bridges {
                vec![0; n]
            } else {
                Vec::new()
            },
            silt: Vec::new(),
            silt_year: 0,
            last_plague: NONE,
            silted_ports: Vec::new(),
            port_price: Vec::new(),
            market_residual: [0.0; 5],
            market_arcs: Vec::new(),
            shadow_arcs: Vec::new(),
            market_unmet: [0.0; 5],
            built: vec![NONE; n],
            ruin: vec![0; n],
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
            famine_until: 0,
            famine_loss: 0.0,
            harvest: Vec::new(),
            harvest_nodes: Vec::new(),
            outside_trade: (0.0, 0.0),
            outside: if p.outside_ports {
                outside_ports(t)
            } else {
                Vec::new()
            },
            sites: (0..n)
                .filter_map(|i| {
                    Good::of(t.resource[i]).map(|good| Site {
                        cell: i as u32,
                        good,
                        holder: NONE,
                        potential: 0.0,
                        output: 0.0,
                        sold: 0.0,
                        fuel_want: 0.0,
                        fuel_got: 0.0,
                    })
                })
                .collect(),
            scratch: Dijkstra::new(n),
            food_cache: FoodCache::default(),
        }
    }

    pub fn index_of(&self, id: u32) -> Option<usize> {
        self.settlements.binary_search_by_key(&id, |s| s.id).ok()
    }

    /// A settlement's name, or `#id` if it no longer exists.
    pub fn name_of(&self, id: u32) -> String {
        self.get(id)
            .map_or_else(|| format!("#{id}"), |s| s.name.clone())
    }

    pub fn get(&self, id: u32) -> Option<&Settlement> {
        self.index_of(id).map(|i| &self.settlements[i])
    }

    pub fn total_pop(&self) -> f32 {
        self.settlements.iter().map(|s| s.pop).sum()
    }

    pub fn rebuild_costs(&mut self, t: &Terrain) {
        self.cost = if self.road_q.is_empty() && self.bridge.is_empty() && self.silt.is_empty() {
            CostGrid::build(t, &self.road, &self.transport)
        } else if self.road_q.is_empty() {
            let q: Vec<f32> = self
                .road
                .iter()
                .map(|&r| if r { 1.0 } else { 0.0 })
                .collect();
            CostGrid::build_with_condition(t, &q, &self.bridge, &self.silt, &self.transport)
        } else {
            CostGrid::build_with_condition(
                t,
                &self.road_q,
                &self.bridge,
                &self.silt,
                &self.transport,
            )
        };
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
        let taken: std::collections::BTreeSet<String> =
            self.settlements.iter().map(|s| s.name.clone()).collect();
        let name = crate::names::settlement_name(t, anchor as usize, id as u64, &taken);
        let mut s = Settlement::new(id, name, anchor, pop, self.year, initial);
        s.market = p.market_w0.min(pop);
        s.history.push(pop);
        self.settlements.push(s);
        let idx = self.settlements.len() - 1;
        crate::reach::refresh_one(self, t, p, idx);
        id
    }
}
