//! Chronicle notes for the grain stores (S3, D81; viewer only): in a year when
//! granaries run dry, "The granaries of X are empty", for the largest place
//! over 500 whose store was at least a tenth of a year's need last year and is
//! spent now.

use crate::view::{Note, NoteKind};
use hinterlands_core::Snapshot;
use std::collections::BTreeMap;

/// Last year's store over need, by settlement.
pub type Fill = BTreeMap<u32, f32>;

pub fn note(s: &Snapshot, last: &mut Fill, notes: &mut Vec<Note>) {
    let mut emptied: Option<(f32, u32, &str)> = None;
    for x in s.settlements.iter() {
        let fill = if x.need > 0.0 { x.store / x.need } else { 0.0 };
        let before = last.get(&x.id).copied().unwrap_or(0.0);
        if x.pop > 500.0 && before >= 0.1 && fill < 0.01 && emptied.is_none_or(|e| x.pop > e.0) {
            emptied = Some((x.pop, x.anchor, &x.name));
        }
    }
    if let Some((_, anchor, name)) = emptied {
        notes.push(Note {
            year: s.year,
            text: format!("The granaries of {name} are empty."),
            anchor,
            kind: NoteKind::Granary,
        });
    }
    last.clear();
    for x in &s.settlements {
        if x.need > 0.0 {
            last.insert(x.id, x.store / x.need);
        }
    }
}
