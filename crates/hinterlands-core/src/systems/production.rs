//! Production at resource sites (v2, D38; Weber 1909). A site is held by the
//! settlement anchored within `min_spacing_km` of it at the least travel cost.
//! Its output, in grain units a year, is `min(cap, prod × (holder's market
//! people + site_crew))`. Iron is fuel-bound: it asks `fuel_per_iron` of forest
//! per unit, which cultivation cuts from the holder's woods, and its output is
//! capped by the fuel cut. With `coppice` (D51) it is also capped by the
//! yearly regrowth above `coppice_floor` on the uncultivated cells of the
//! holder's fuel catchment. Writes: sites (holder, potential, output, fuel_want).

use crate::params::Params;
use crate::state::{Good, State, NONE};
use crate::terrain::Terrain;
use crate::Rng;

pub fn cap(p: &Params, g: Good) -> f32 {
    match g {
        Good::Salt => p.cap_salt,
        Good::Iron => p.cap_iron,
        Good::Stone => p.cap_stone,
    }
}

pub fn prod(p: &Params, g: Good) -> f32 {
    match g {
        Good::Salt => p.prod_salt,
        Good::Iron => p.prod_iron,
        Good::Stone => p.prod_stone,
    }
}

pub fn run(state: &mut State, t: &Terrain, p: &Params, _rng: &mut Rng) {
    let State {
        settlements,
        sites,
        owner,
        ..
    } = state;
    for site in sites.iter_mut() {
        site.holder = NONE;
        site.potential = 0.0;
        site.output = 0.0;
        site.fuel_want = 0.0;
        if !p.goods_enabled {
            continue;
        }
        let cell = site.cell as usize;
        let holder = settlements
            .iter()
            .filter(|s| t.grid.dist_km(s.anchor as usize, cell) <= p.min_spacing_km)
            .map(|s| (s.reach.cost_to_cell(cell), s.id, s))
            .filter(|(c, _, _)| c.is_finite())
            .min_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let Some((_, id, s)) = holder else { continue };
        let workers = s.market.max(0.0) + p.site_crew;
        site.holder = id;
        site.potential = cap(p, site.good).min(prod(p, site.good) * workers);
        site.output = site.potential;
        if site.good == Good::Iron {
            let fpi = p.fuel_per_iron.max(1e-9);
            let mut most = site.potential;
            if p.coppice {
                let regrowth: f32 = s
                    .reach
                    .fuel_order
                    .iter()
                    .map(|&c| c as usize)
                    .filter(|&c| owner[c] == NONE)
                    .map(|c| p.regrow_rate * (t.forest0[c] - p.coppice_floor).max(0.0))
                    .sum();
                most = most.min(regrowth / fpi);
            }
            site.output = most.min(site.fuel_got / fpi);
            site.fuel_want = most * p.fuel_per_iron;
        }
    }
    // The sites each place works this year (D1: tools from stocks).
    for s in settlements.iter_mut() {
        s.works = sites
            .iter()
            .filter(|x| x.holder == s.id && x.output > 0.0)
            .count() as u32;
    }
}
