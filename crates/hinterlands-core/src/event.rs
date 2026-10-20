//! The event queue. Shocks, planted towns and bridges arrive here in later
//! versions; v1 defines the type and applies nothing.

use crate::state::State;
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Event {}

impl Event {
    pub fn apply(&self, _state: &mut State, _t: &Terrain) {
        match *self {}
    }
}
