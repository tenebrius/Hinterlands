//! The read-only view of a year. The viewer and the CLI only ever see this.

use crate::cost::Transport;
use crate::state::{Good, Kind, LogEntry, Rank, State, Tally};
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
    /// Specialist kind (v2); left out of the bytes for ordinary settlements.
    #[serde(default = "agrarian", skip_serializing_if = "is_agrarian")]
    pub kind: Kind,
    /// Grain received for goods sold this year (v2); left out when zero.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub goods_in: f32,
    /// Has a mill (v3, D56); left out of the bytes when false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub mill: bool,
    /// Planted by a lord (v4, D63); left out of the bytes when false.
    #[serde(default, skip_serializing_if = "is_false")]
    pub planted: bool,
    /// Shadow prices (v6 S0): grain, salt, iron, stone, imports; left out of
    /// the bytes unless `market_shadow` is on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<[f32; 5]>,
    /// Grain in store (S3, D81); left out of the bytes when zero.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub store: f32,
    /// This year's grain price at the place's market (v7); left out when zero.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub grain_price: f32,
    /// Market people (W, v1.1) and the market's store (M3); left out when zero.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub market: f32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub mstore: f32,
    /// The farmers' and the market's purses (D96); left out when zero.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub purse: f32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub mpurse: f32,
    /// Cells grazed for wool (L1); left out when zero.
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub pasture: u32,
    /// Coin its wool fetched this year, lords' and farmers' shares (D2, v8);
    /// left out when zero.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub wool_got: f32,
    /// Export cloth made this year at its mill, lb of wool (D2, v8).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub cloth_made: f32,
    /// The cloth's margin over its wool this year, coin (D2, v8).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub cloth_margin: f32,
}

fn is_zero_u32(x: &u32) -> bool {
    *x == 0
}

fn is_false(b: &bool) -> bool {
    !*b
}

fn agrarian() -> Kind {
    Kind::Agrarian
}

fn is_agrarian(k: &Kind) -> bool {
    *k == Kind::Agrarian
}

fn is_zero(x: &f32) -> bool {
    *x == 0.0
}

/// A resource site as the viewer sees it (v2).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteView {
    pub cell: u32,
    pub good: Good,
    /// The settlement working the site, or `NONE`.
    pub holder: u32,
    /// Capacity this year, grain units.
    pub capacity: f32,
    /// Sold this year, its holder's own use included.
    pub sold: f32,
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
    /// Imports from an outside port (v5); left out of the bytes when false.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub import: bool,
    /// Grain relief (S2′, D80); left out of the bytes when false.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub relief: bool,
    /// Wool to an outside port (L1); left out of the bytes when false.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub wool: bool,
    /// Export cloth from a mill to an outside port (D2, v8); left out of the
    /// bytes when false.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub cloth: bool,
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
    /// Resource sites (v2); left out of the bytes with goods off.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sites: Vec<SiteView>,
    /// Cells the model has bridged (v3, D55), ascending; left out when none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bridges: Vec<u32>,
    /// Anchors of silted harbours (v4, D60), in order; left out when none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub silted: Vec<u32>,
    /// Outside port cells (v5, D64); left out when there are none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<u32>,
    /// Pasture cells (L1), ascending; left out when there are none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pasture: Vec<u32>,
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
                kind: s.kind,
                goods_in: s.goods_in,
                mill: s.mill,
                planted: s.planted,
                price: p.market_shadow.then_some(s.shadow_price),
                store: s.store + s.mstore,
                grain_price: s.price[0],
                market: s.market,
                mstore: s.mstore,
                purse: s.purse,
                mpurse: s.mpurse,
                pasture: s.pasture.len() as u32,
                wool_got: s.wool_got,
                cloth_made: s.cloth_made,
                cloth_margin: s.cloth_margin,
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
                    path: s.reach.partner(f.partner_idx).path.clone(),
                    good: f.good,
                    payment: f.payment,
                    import: f.import,
                    relief: f.relief,
                    wool: f.wool,
                    cloth: f.cloth,
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
            sites: if p.goods_enabled {
                state
                    .sites
                    .iter()
                    .map(|s| SiteView {
                        cell: s.cell,
                        good: s.good,
                        holder: s.holder,
                        capacity: s.output,
                        sold: s.sold,
                    })
                    .collect()
            } else {
                Vec::new()
            },
            silted: state.silted_ports.clone(),
            ports: state
                .outside
                .iter()
                .map(|&node| node - t.n() as u32)
                .collect(),
            bridges: (0..state.bridge.len())
                .filter(|&i| state.bridge[i])
                .map(|i| i as u32)
                .collect(),
            pasture: (0..state.grazed.len())
                .filter(|&i| state.grazed[i] != crate::state::NONE)
                .map(|i| i as u32)
                .collect(),
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
