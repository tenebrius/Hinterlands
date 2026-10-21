//! The read-only view of a year. The viewer and the CLI only ever see this.

use crate::cost::Transport;
use crate::state::{LogEntry, Rank, State, Tally};
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};

/// Flows below this are left out of the snapshot (they are still simulated).
pub const FLOW_VIEW_MIN: f32 = 1.0;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SettlementView {
    pub id: u32,
    pub name: String,
    pub anchor: u32,
    pub pop: f32,
    pub rank: Rank,
    pub founded: u32,
    pub initial: bool,
    pub food_prod: f32,
    pub need: f32,
    pub food_in: f32,
    pub food_out: f32,
    pub trade_in: f32,
    pub through: f32,
    pub food_ratio: f32,
    pub fields: u32,
    /// Population 50 years ago (or at founding, if younger).
    pub pop_50_ago: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowView {
    pub from: u32,
    pub to: u32,
    pub amount: f32,
    pub food: f32,
    pub path: Vec<u32>,
    /// The good carried (v2); absent for grain, and left out of the bytes then.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub good: Option<crate::state::Good>,
    /// Grain paid for goods (v2).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub payment: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub year: u32,
    pub terrain_hash: u64,
    pub transport: Transport,
    /// Forest cover in 1/255 steps.
    pub forest: Vec<u8>,
    pub owner: Vec<u32>,
    pub road: Vec<bool>,
    pub road_edges: Vec<(u32, u32)>,
    pub built: Vec<u32>,
    pub ruin: Vec<u8>,
    pub hub: Vec<bool>,
    pub site_score: Vec<f32>,
    pub settlements: Vec<SettlementView>,
    pub flows: Vec<FlowView>,
    pub log: Vec<LogEntry>,
    pub tally: Tally,
}

impl Snapshot {
    pub fn of(state: &State, t: &Terrain, terrain_hash: u64, p: &crate::Params) -> Self {
        let _ = t;
        let settlements = state
            .settlements
            .iter()
            .map(|s| SettlementView {
                id: s.id,
                name: s.name.clone(),
                anchor: s.anchor,
                pop: s.pop,
                rank: s.rank(),
                founded: s.founded,
                initial: s.initial,
                food_prod: s.food_prod,
                need: s.need(p),
                food_in: s.food_in,
                food_out: s.food_out,
                trade_in: s.trade_in,
                through: s.through,
                food_ratio: s.food_ratio,
                fields: s.fields.len() as u32,
                pop_50_ago: s.history.first().copied().unwrap_or(s.pop),
            })
            .collect();
        let flows = state
            .flows
            .iter()
            .filter(|f| f.amount >= FLOW_VIEW_MIN)
            .filter_map(|f| {
                let s = state.get(f.from)?;
                Some(FlowView {
                    from: f.from,
                    to: f.to,
                    amount: f.amount,
                    food: f.food,
                    path: s.reach.partners[f.partner_idx as usize].path.clone(),
                    good: f.good,
                    payment: f.payment,
                })
            })
            .collect();
        Self {
            year: state.year,
            terrain_hash,
            transport: state.transport,
            forest: state
                .forest
                .iter()
                .map(|&f| (f.clamp(0.0, 1.0) * 255.0).round() as u8)
                .collect(),
            owner: state.owner.clone(),
            road: state.road.clone(),
            road_edges: state.road_edges.clone(),
            built: state.built.clone(),
            ruin: state.ruin.clone(),
            hub: state.hub.clone(),
            site_score: state.site_score.clone(),
            settlements,
            flows,
            log: state.log.clone(),
            tally: state.tally.clone(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("snapshot serialises")
    }

    pub fn from_bytes(b: &[u8]) -> Result<Self, String> {
        bincode::deserialize(b).map_err(|e| e.to_string())
    }

    pub fn hash(&self) -> u64 {
        crate::fnv1a(&self.to_bytes())
    }

    pub fn total_pop(&self) -> f32 {
        self.settlements.iter().map(|s| s.pop).sum()
    }

    pub fn settlement(&self, id: u32) -> Option<&SettlementView> {
        self.settlements.iter().find(|s| s.id == id)
    }
}
