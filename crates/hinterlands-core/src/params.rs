//! Every model parameter, with its default, range and group. The `define_params!`
//! macro generates the struct, its defaults and a field listing, so the viewer
//! builds its sliders from `Params::INFO` and a new parameter is one line here.

use serde::{Deserialize, Serialize};

pub trait ParamNum: Copy {
    const INTEGER: bool;
    const BOOL: bool;
    fn to_f64(self) -> f64;
    fn from_f64(v: f64) -> Self;
}

impl ParamNum for f32 {
    const INTEGER: bool = false;
    const BOOL: bool = false;
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as f32
    }
}

impl ParamNum for u32 {
    const INTEGER: bool = true;
    const BOOL: bool = false;
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(v: f64) -> Self {
        v.round().max(0.0) as u32
    }
}

impl ParamNum for bool {
    const INTEGER: bool = false;
    const BOOL: bool = true;
    fn to_f64(self) -> f64 {
        if self {
            1.0
        } else {
            0.0
        }
    }
    fn from_f64(v: f64) -> Self {
        v >= 0.5
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParamInfo {
    pub name: &'static str,
    pub group: &'static str,
    pub doc: &'static str,
    pub min: f64,
    pub max: f64,
    pub integer: bool,
    pub boolean: bool,
}

macro_rules! define_params {
    ($( $group:literal => { $( $name:ident : $ty:ty = $def:expr, [$min:expr, $max:expr], $doc:literal; )* } )*) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        pub struct Params {
            $( $( #[doc = $doc] pub $name: $ty, )* )*
        }

        impl Default for Params {
            fn default() -> Self {
                Self { $( $( $name: $def, )* )* }
            }
        }

        impl Params {
            pub const INFO: &'static [ParamInfo] = &[
                $( $( ParamInfo {
                    name: stringify!($name),
                    group: $group,
                    doc: $doc,
                    min: $min as f64,
                    max: $max as f64,
                    integer: <$ty as ParamNum>::INTEGER,
                    boolean: <$ty as ParamNum>::BOOL,
                }, )* )*
            ];

            pub fn get(&self, name: &str) -> Option<f64> {
                match name {
                    $( $( stringify!($name) => Some(ParamNum::to_f64(self.$name)), )* )*
                    _ => None,
                }
            }

            /// Set a field by name. Returns false for an unknown name.
            pub fn set(&mut self, name: &str, v: f64) -> bool {
                match name {
                    $( $( stringify!($name) => { self.$name = <$ty as ParamNum>::from_f64(v); true } )* )*
                    _ => false,
                }
            }
        }
    };
}

define_params! {
    "Start" => {
        n_initial: u32 = 10, [1, 30], "Number of starting hamlets";
        initial_pop: f32 = 50.0, [10, 500], "Population of each starting hamlet";
        initial_spacing_km: f32 = 8.0, [2, 30], "Minimum distance between starting hamlets, km";
    }
    "Transport" => {
        cost_land: f32 = 1.0, [0.2, 3], "Land travel cost per km on the flat";
        cost_road: f32 = 0.5, [0.1, 2], "Road travel cost per km";
        cost_river_down: f32 = 0.2, [0.02, 2], "Navigable river cost per km, downstream";
        cost_river_up: f32 = 0.4, [0.02, 3], "Navigable river cost per km, upstream";
        cost_sea: f32 = 0.08, [0.01, 1], "Sea cost per km";
        cost_river_cross: f32 = 5.0, [0, 20], "Fixed cost to cross a navigable river away from a ford";
        cost_stream_cross: f32 = 1.0, [0, 10], "Fixed cost to cross a stream away from a ford";
        cost_transship: f32 = 3.0, [0, 20], "Fixed cost to step between land and boat at a harbour or river";
        cost_river_sea_transship: f32 = 3.8, [0, 20], "Fixed cost to move cargo between river barge and sea ship at a mouth";
        open_coast_factor: f32 = 2.0, [1, 5], "Transshipment multiplier on open (non-harbour) coast";
        tobler: bool = true, [0, 1], "Scale land costs by Tobler's slope factor";
    }
    "Cultivation" => {
        cultivation_enabled: bool = true, [0, 1], "Run the cultivation system";
        food_per_person: f32 = 1.0, [0.5, 2], "Food units one person needs per year";
        people_per_prime_cell: f32 = 150.0, [50, 400], "People fed by one cell of prime land";
        r_cult: f32 = 1.3, [1, 12], "Travel-cost radius of cultivation";
        cult_margin: f32 = 1.2, [1, 2], "Fields are claimed until produce covers need times this";
        release_hysteresis: f32 = 0.1, [0, 0.5], "Fields are released only when produce exceeds the target by this share";
        min_field_yield: f32 = 0.1, [0, 0.8], "Cells below this yield are never cultivated";
        clear_rate: f32 = 0.2, [0, 1], "Forest lost per year on a cultivated cell";
        regrow_rate: f32 = 0.02, [0, 0.2], "Forest regained per year on an uncultivated cell";
        fuel_per_capita: f32 = 0.00070602755, [0, 0.002], "Forest cover (cell units) burned per person per year";
        r_fuel: f32 = 8.0, [1, 20], "Travel-cost radius of fuel cutting";
        max_fuel_take: f32 = 0.1, [0.01, 1], "Most forest cover cut from one cell per year";
        resource_output: f32 = 50.0, [0, 500], "Trade output of an owned resource cell per year";
    }
    "Trade" => {
        trade_enabled: bool = true, [0, 1], "Run the trade system";
        c_max: f32 = 30.0, [10, 200], "Largest travel cost at which two settlements trade";
        alpha: f32 = 1.0882609, [0.5, 2], "Size exponent of the gravity attraction";
        beta: f32 = 0.13902539, [0.005, 0.5], "Distance decay of trade per km-equivalent";
        transport_loss: f32 = 0.005, [0, 0.03], "Share of shipped food lost per unit cost";
        cost_refresh_years: u32 = 5, [1, 25], "Years between refreshes of travel costs and paths";
        hub_min_flow: f32 = 5.0, [0, 200], "Least flow on a route for it to count towards a hub";
        home_weight: f32 = 1.0, [0, 2], "Weight of a settlement's own market in its trade (Rihll-Wilson self-term)";
        urban_seed: f32 = 10.0, [0, 500], "Market pull of a settlement with no non-farming population";
        market_share: f32 = 0.48655713, [0, 0.6], "Share of each harvest sent to market (rents, tithes, sales)";
        outside_enabled: bool = false, [0, 1], "An outside world off the sea edge trades with the region (D25)";
        outside_size: f32 = 5000.0, [0, 100000], "Market size W of the outside world";
        outside_beta: f32 = 0.05, [0.005, 0.5], "Distance decay of trade with the outside world";
        outside_out: f32 = 2000.0, [0, 50000], "Goods the outside world sends into the region per year";
    }
    "Goods" => {
        goods_enabled: bool = true, [0, 1], "Goods besides grain: resource sites, goods trade and specialists (v2, D38-D40)";
        value_salt: f32 = 1.0, [0.1, 10], "Grain paid per unit of salt";
        value_iron: f32 = 1.0, [0.1, 10], "Grain paid per unit of iron";
        value_stone: f32 = 1.0, [0.1, 10], "Grain paid per unit of stone";
        rho_salt: f32 = 3.0, [0.1, 20], "Value density of salt relative to grain; its distance decay is beta / rho";
        rho_iron: f32 = 8.0, [0.1, 20], "Value density of iron relative to grain";
        rho_stone: f32 = 0.4, [0.1, 20], "Value density of stone relative to grain";
        demand_salt: f32 = 0.02, [0, 0.2], "Salt wanted per person a year, grain units";
        demand_iron: f32 = 0.02, [0, 0.2], "Iron wanted per person a year, grain units";
        demand_stone: f32 = 0.01603657, [0, 0.3], "Stone wanted per market person a year, grain units";
        prod_salt: f32 = 2.0, [0, 20], "Salt output per worker a year, grain units";
        prod_iron: f32 = 2.0, [0, 20], "Iron output per worker a year, grain units";
        prod_stone: f32 = 2.0, [0, 20], "Stone output per worker a year, grain units";
        site_crew: f32 = 10.0, [0, 100], "Workers at a held site besides the holder's market people";
        cap_salt: f32 = 1000.0, [0, 10000], "Most output of one salt site a year";
        cap_iron: f32 = 1000.0, [0, 10000], "Most output of one iron site a year";
        cap_stone: f32 = 1000.0, [0, 10000], "Most output of one stone site a year";
        fuel_per_iron: f32 = 0.1, [0, 1], "Forest (cell units) burned per unit of iron";
        coppice: bool = true, [0, 1], "Iron fuel is coppiced: cut only down to coppice_floor, output capped by the regrowth above it (D51)";
        coppice_floor: f32 = 0.3, [0, 1], "Forest cover iron fuel cutting leaves standing (coppice)";
        specialist_share: f32 = 0.25, [0, 1], "A specialist stays one while goods sales are at least this share of its gross inflow (10-year average)";
    }
    "Growth" => {
        growth_enabled: bool = true, [0, 1], "Run the growth system";
        r_grow: f32 = 0.027, [0, 0.1], "Yearly natural growth at a full food surplus";
        r_shrink: f32 = 0.02, [0, 0.2], "Yearly decline per unit of food shortfall";
        food_cap: f32 = 1.2, [1, 2], "Food ratio above which surplus adds no natural growth";
        epsilon: f32 = 0.13734761, [0, 0.5], "Response rate of size to inflow (Rihll-Wilson)";
        k_maint: f32 = 3.3905132, [0.2, 3], "Inflow needed per person to hold size (Rihll-Wilson K)";
        through_weight: f32 = 0.025, [0, 2], "Weight of trade passing through and non-food trade in inflow";
        gross_inflow: bool = false, [0, 1], "Markets grow on gross market inflow, as published Rihll-Wilson, not net food imports (D24)";
        market_state: bool = true, [0, 1], "Market size W is its own state, grown on gross market inflow (Rihll-Wilson; D26)";
        market_w0: f32 = 4.6169, [0, 200], "Market size of a new settlement, in people";
        market_floor: f32 = 1.0, [0.01, 50], "Least market size, so a losing market keeps a defined pull";
        handling_income: bool = true, [0, 1], "Places earn a margin on trade passing or transshipped there, as food-equivalent inflow (D27)";
        handling_margin: f32 = 0.04567222, [0, 0.5], "Share of the value of trade passing a place's anchor that it keeps";
        transship_margin: f32 = 0.23906788, [0, 1], "Share of the value of trade transshipped at a place that it keeps";
        fed_ratio: f32 = 0.9, [0.5, 1], "Food ratio at or above which a settlement counts as fed";
        max_growth: f32 = 0.05, [0.005, 0.2], "Cap on yearly growth as a share of population";
        migration_cap: f32 = 0.06, [0, 0.2], "Most a settlement can lose to town migration in a year, as a share";
        shadow_drain: f32 = 0.0168, [0, 0.1], "Yearly share lost to each larger neighbour in the shadow, per 1,000 of its people";
        shadow_size_ratio: f32 = 3.0, [1, 20], "A neighbour drains only if this many times larger";
        loss_years: f32 = 3.0, [1, 10], "Years over which a food-driven loss is spread";
    }
    "Founding" => {
        founding_enabled: bool = true, [0, 1], "Run the founding system";
        founding_interval: u32 = 5, [1, 25], "Years between founding steps";
        w_hub: f32 = 0.0, [0, 20], "Score weight of a route hub";
        w_river: f32 = 0.0, [0, 5], "Score weight of a navigable river";
        w_coast: f32 = 0.0, [0, 5], "Score weight of the coast";
        w_road: f32 = 0.0, [0, 5], "Score weight of a road";
        w_food: f32 = 1.0950333, [0, 5], "Score weight of free food within the cultivation radius";
        shadow_weight: f32 = 4.3475847, [0, 10], "Urban shadow per 1,000 people within the shadow radius";
        shadow_radius: f32 = 10.0, [0, 40], "Travel cost within which a settlement casts a shadow (without shadow_km)";
        shadow_km: bool = true, [0, 1], "The urban shadow, in founding and in the growth drain, is measured in straight-line km between anchors (D52)";
        shadow_radius_km: f32 = 10.0, [0, 40], "Distance, km, within which a settlement casts a shadow (with shadow_km)";
        access_weight: f32 = 0.3, [0, 5], "Market access per 1,000 people beyond the shadow radius";
        access_decay: f32 = 0.05, [0, 0.5], "Decay of market access per unit cost";
        noise: f32 = 0.3083304, [0, 2], "Upper bound of the uniform noise term";
        found_threshold: f32 = 0.3269517, [0, 10], "Least score for a new settlement";
        max_found_per_step: u32 = 6, [0, 30], "Most settlements founded per founding step";
        min_spacing_km: f32 = 3.0, [1, 20], "No founding within this distance of a settlement, km";
        water_radius_km: f32 = 1.0, [0, 5], "Founding sites need water within this distance, km";
        spring_water: bool = true, [0, 1], "Water supply at brook scale: founding allowed near small streams, water scored by adjacency (D29)";
        brook_area: f32 = 5.0, [0.5, 60], "Drainage (km²) at which a cell carries a brook a village can draw on";
        supply_radius_km: f32 = 3.0, [0, 10], "Founding needs a brook or the coast within this distance, km";
        w_water_cell: f32 = 1.1378639, [0, 3], "Score bonus for a brook or river in the cell";
        w_water_near: f32 = 0.42684895, [0, 3], "Score bonus for a brook, river or the sea within 1 km";
        found_pop: f32 = 30.0, [5, 200], "Population of a new hamlet";
        donor_min_ratio: f32 = 1.1, [0.5, 2], "Least food ratio of the settlement that sends migrants";
    }
    "Failure" => {
        failure_enabled: bool = true, [0, 1], "Run the failure system";
        fail_pop: f32 = 20.0, [0, 200], "Population below which a settlement counts as failing";
        fail_low_years: u32 = 12, [1, 100], "Years below the failing population before removal";
        fail_unfed_years: u32 = 17, [1, 100], "Years unfed before removal";
        shed_unfed: bool = true, [0, 1], "Underfed places shed a third of their food gap a year instead of being removed when unfed (D34)";
    }
    "View" => {
        density_per_cell: f32 = 1500.0, [100, 20000], "People per built-up cell in the drawn footprint (rendering only)";
        ruin_years: u32 = 60, [0, 255], "Years an abandoned built-up cell stays visible as a ruin";
    }
    "Roads" => {
        roads_enabled: bool = true, [0, 1], "Run the roads system";
        road_interval: u32 = 10, [1, 50], "Years between road building steps";
        road_pairs: u32 = 20, [0, 100], "Trading pairs given a road per step";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_lists_every_field_and_roundtrips() {
        let mut p = Params::default();
        for info in Params::INFO {
            let v = p.get(info.name).unwrap();
            assert!(p.set(info.name, v));
        }
        assert_eq!(p, Params::default());
        assert!(!p.set("nope", 1.0));
    }
}
