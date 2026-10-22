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
        market_share: f32 = 0.35, [0, 0.6], "Share of each harvest sent to market (rents, tithes, sales)";
    }
    "Goods" => {
        goods_enabled: bool = true, [0, 1], "Goods besides grain: resource sites, goods trade and specialists (v2, D38-D40)";
        value_salt: f32 = 1.0, [0.1, 10], "Grain paid per unit of salt";
        value_iron: f32 = 1.0, [0.1, 10], "Grain paid per unit of iron";
        value_stone: f32 = 1.0, [0.1, 10], "Grain paid per unit of stone";
        rho_salt: f32 = 3.0, [0.1, 20], "Value density of salt relative to grain; its distance decay is beta / rho";
        rho_iron: f32 = 8.0, [0.1, 20], "Value density of iron relative to grain";
        rho_stone: f32 = 0.4, [0.1, 20], "Value density of stone relative to grain";
        port_goods: bool = false, [0, 1], "The outside ports sell salt and iron, as they sell imports (D1 step 1, D121)";
        demand_stocks: bool = true, [0, 1], "Demand for salt, iron and stone from stocks, and ports selling salt and iron (D1, D121)";
        salt_lb: f32 = 15.0, [0, 100], "Salt a person uses a year, lb (calibration: no source found, D121)";
        salt_lb_per_d: f32 = 3.1, [0.1, 100], "Pounds of salt a penny buys (London 1438, secondary)";
        ploughs_per_km2: f32 = 2.0, [0, 10], "Ploughs per km2 of arable (a ploughland of about 100 acres; approximate)";
        plough_iron_d: f32 = 36.0, [0, 500], "Iron a plough's irons and shoeing cost a year, pence (shoeing after Langdon 1982; plough irons a calibration)";
        household_iron_d: f32 = 1.0, [0, 50], "Household iron a person buys a year, pence (calibration)";
        site_iron_d: f32 = 24.0, [0, 500], "Tools a worked site buys a year, pence (calibration)";
        stone_repair: f32 = 0.02, [0.001, 0.5], "Share of the building stock repaired a year; the stock per market person is demand_stone over this (calibration, D0)";
        demand_salt: f32 = 0.02, [0, 0.2], "Salt wanted per person a year, grain units";
        demand_iron: f32 = 0.0013, [0, 0.2], "Iron wanted per person a year, grain units";
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
        k_maint: f32 = 1.3, [0.2, 3], "Inflow needed per person to hold size (Rihll-Wilson K)";
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
        shadow_drain: f32 = 0.045, [0, 0.1], "Yearly share lost to each larger neighbour in the shadow, per 1,000 of its people";
        drain_radius_km: f32 = 8.0, [0, 40], "Distance, km, within which a much larger neighbour drains a place (with shadow_km, D53)";
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
        shadow_weight: f32 = 4.5326834, [0, 10], "Urban shadow per 1,000 people within the shadow radius";
        shadow_radius: f32 = 10.0, [0, 40], "Travel cost within which a settlement casts a shadow (without shadow_km)";
        shadow_km: bool = false, [0, 1], "The urban shadow, in founding and in the growth drain, is measured in straight-line km between anchors (D52)";
        shadow_radius_km: f32 = 10.0, [0, 40], "Distance, km, within which a settlement casts a founding shadow (with shadow_km)";
        access_weight: f32 = 0.3, [0, 5], "Market access per 1,000 people beyond the shadow radius";
        access_decay: f32 = 0.05, [0, 0.5], "Decay of market access per unit cost";
        noise: f32 = 0.3244266, [0, 2], "Upper bound of the uniform noise term";
        found_threshold: f32 = 0.44928834, [0, 10], "Least score for a new settlement";
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
        wool_price_sack: f32 = 5.0, [0, 30], "World price of wool, pounds sterling a sack of 364 lb (Lloyd 1294 mean, D107)";
        wool_fall: f32 = 0.25, [0, 1], "Fall in the wool price against wheat after the plague (Clark 2004: about a quarter; D109)";
        wool_fall_from: u32 = 250, [0, 2000], "Year the wool price starts to fall (1350; D109)";
        wool_fall_to: u32 = 300, [0, 2000], "Year it reaches its lower level (1400; D109)";
        murrain: bool = true, [0, 1], "The sheep murrain of 1315-22 in the Historical calendar (L2, D112)";
        murrain_year: u32 = 215, [0, 2000], "First murrain year (1315)";
        murrain_loss: f32 = 0.15, [0, 1], "Share of wool lost in the murrain years (1315-22; approximate, after Slavin 2020; D114)";
        murrain_years: u32 = 8, [0, 20], "Years at the first loss";
        murrain_late_loss: f32 = 0.1, [0, 1], "Share of wool lost to scab in the years after (1323-32; approximate, after Slavin 2020; D114)";
        murrain_late_years: u32 = 10, [0, 20], "Years at the later loss";
        grass_log_cells: u32 = 3, [1, 100], "Old fields a place must put to grass in a year to make the chronicle (L4)";
        grass_log_gap: u32 = 50, [1, 500], "Least years between a place's grass lines in the chronicle (L4)";
        grass_log_forest: f32 = 0.1, [0, 1], "Forest cover below which a cell counts as an old field (L4)";
        sheep_per_acre: f32 = 1.5, [0, 5], "Sheep a pasture acre carries (Howell; L0)";
        fleece_lb: f32 = 1.5, [0, 5], "Wool a fleece, lb (200-260 fleeces to the sack; L0)";
        wheat_s_bu: f32 = 0.6, [0.1, 5], "Wheat price, shillings a bushel, 1300s (Clark 2004)";
        bu_per_person: f32 = 6.0, [1, 20], "Bushels of wheat in a person-year of grain (Campbell's 1,500 grain kcal a day; L0)";
        rho_wool: f32 = 50.0, [0.1, 200], "Value density of wool relative to grain (Clark's prices; D107)";
        pasture_labour: f32 = 5.0, [0, 100], "People a km2 of pasture needs: a shepherd's household (calibration, D107)";
        arable_labour: f32 = 40.0, [0, 200], "People a km2 of arable needs, for the labour left to herd (calibration from L0's 39-45, L1)";
        hunger_tolerance: f32 = 0.8, [0.5, 1], "Food ratio above which a shortfall stops growth but sheds nobody (D101, D104)";
        shed_unfed: bool = true, [0, 1], "Underfed places shed a third of their food gap a year instead of being removed when unfed (D34)";
    }
    "Outside" => {
        outside_ports: bool = true, [0, 1], "Outside ports at the open-sea map edges buy grain and sell imports (v5, D64)";
        outside_grain: f32 = 700.0, [0, 100000], "Grain each outside port buys a year, grain units";
        import_demand: f32 = 0.07, [0, 2], "Imports wanted a year per market person, grain units";
        rho_import: f32 = 10.0, [0.1, 50], "Value density of imports: distance decay is beta / rho";
    }
    "Market" => {
        market_goods: bool = true, [0, 1], "Salt, iron, stone and imports trade by the price equilibrium instead of the shopping model (v6 S1, D71)";
        grain_stores: bool = true, [0, 1], "Each place keeps a grain store, filled from surplus above food_cap and drawn in deficit, own first, then by neighbours (v6 S3, D81)";
        store_cap: f32 = 1.0, [0, 2], "Largest store, as a share of a year's need (S3)";
        pull_smoothed: bool = true, [0, 1], "The market pull in growth answers the ten-year average inflow instead of the year's (W2, D85)";
        store_smooth: bool = false, [0, 1], "A place tops its food up from its own store to food_cap x need, its growth target, not just to need (W1, D83)";
        store_loss: f32 = 0.05, [0, 0.5], "Share of a store lost each year to spoilage, vermin and holding (S3)";
        market_min: f32 = 100.0, [1, 1000], "Least market size W for a market to trade with other markets at a price (v7)";
        market_stores: bool = true, [0, 1], "Large markets hold grain stores, buying below their ten-year average price less a holding margin and selling above it (v7 M3, D94)";
        store_gain: f32 = 2.0, [0, 20], "Store purchases or sales per unit of relative price gap, as a share of the market's yearly turnover (M3)";
        store_margin_early: f32 = 0.15, [0, 0.5], "Holding margin (interest plus storage) before store_margin_year (Clark)";
        store_margin_late: f32 = 0.10, [0, 0.5], "Holding margin from store_margin_year on (Clark)";
        store_avg_years: f32 = 10.0, [1, 50], "Years over which a market's reference grain price is averaged for its store (D94)";
        relief_market_stores: bool = true, [0, 1], "Relief may draw on market stores as well as village stores (experiment, D94)";
        purse_loss: f32 = 0.05, [0, 0.5], "Share of the farmers' purse spent each year with their market (dues, cloth, ale, the church; D97)";
        purse_cap: f32 = 2.0, [0, 5], "Largest farmers' purse, in years of the farmers' need at price 1";
        mpurse_loss: f32 = 0.1, [0, 0.5], "Share of the market's purse spent each year on the farmers it buys from (building, service, alms; D97)";
        mpurse_cap: f32 = 2.0, [0, 10], "Largest market purse, in years of its market people's consumption at price 1";
        luxury_out: f32 = 0.2, [0, 1], "Share of the market's yearly spending that leaves the region as imported luxuries (D97)";
        coin_wear: f32 = 0.01, [0, 0.2], "Share of all coin lost each year to wear and hoards (D97)";
        purse_start: f32 = 0.5, [0, 5], "Starting hamlets' purses, in years of need (D96)";
        planted_coin: f32 = 0.5, [0, 5], "Coin a planted town brings from outside, in years of its need (D96)";
        store_spike: bool = true, [0, 1], "Stores sell only when the price also exceeds last year's by the holding margin: into a spike, not a rising trend (D94)";
        store_sell_band: f32 = 0.0, [0, 3], "Stores sell above the average price plus this many holding margins (0: above the average, D94)";
        store_margin_year: u32 = 250, [0, 2000], "Year the holding margin falls (historical; 1350)";
        toll: f32 = 0.1, [0, 0.2], "Toll on the turnover of a market, paid to its market people (v7)";
        grain_relief: bool = true, [0, 1], "After trade, places in deficit draw on surplus above food_cap that would otherwise be lost, unpaid (v6 S2', D80)";
        market_grain: bool = false, [0, 1], "Grain trades by the price equilibrium, paid from budgets of sales, rents, fees and goods (v6 S2, D78)";
        grain_premium: bool = true, [0, 1], "Under market_grain, market size W^alpha is a premium on the buyer side (decision 4; off is option 1, W as demand only)";
        rent_share: f32 = 0.48655713, [0, 0.9], "Under market_grain, share of each place's grain sales paid as rent and tithe to markets, in value (D77)";
        p_floor: f32 = 0.25, [0.01, 1], "Under market_grain, least supply price of grain; what does not sell at it is lost (D77)";
        port_world_price: f32 = 0.6, [0.05, 3], "Under market_grain, the world grain price: the outside ports buy their fixed grain up to it (D78)";
        port_elast: f32 = 4.0, [0, 20], "Under market_grain, price elasticity of the outside ports' grain demand above the world price";
        grain_flow_min: f32 = 0.05, [0, 10], "Under market_grain, least grain flow assigned to a path";
        market_shadow: bool = false, [0, 1], "Solve shadow prices each year beside the trade; nothing feeds back (v6 S0, D69)";
        market_tau: f32 = 0.002, [0.0001, 0.05], "Transport price of grain per unit of travel cost, as a share of its value (Clark: about 0.2% per km)";
        market_passes: u32 = 8, [1, 200], "Balancing passes a year";
        market_damping: f32 = 0.7, [0.05, 1], "Share of each Newton step taken";
        market_step: f32 = 1.0, [0.01, 2], "Largest change in log price per pass";
        elast_grain: f32 = 0.75, [0, 3], "Price elasticity of grain demand (Clark)";
        elast_est: f32 = 1.5, [0, 5], "Price elasticity of the market establishment's grain (malt, fodder): it gives way first in dearth (D99)";
        elast_salt: f32 = 0.2, [0, 3], "Price elasticity of salt demand";
        elast_iron: f32 = 0.5, [0, 3], "Price elasticity of iron demand";
        elast_stone: f32 = 1.0, [0, 3], "Price elasticity of stone demand";
        elast_import: f32 = 1.0, [0, 3], "Price elasticity of import demand";
    }
    "Events" => {
        events_mode: u32 = 1, [0, 2], "Shocks: 0 off, 1 historical calendar (year 0 = 1100), 2 random (D57)";
        famine_year: u32 = 215, [0, 2000], "Year the Great Famine begins (historical; 1315)";
        famine_years: u32 = 3, [1, 10], "Years a famine lasts";
        famine_loss: f32 = 0.4, [0, 1], "Share of every cell's yield lost in a famine's first year (1315: −40%)";
        famine_loss_2: f32 = 0.6, [0, 1], "Share lost in its second year (1316: −60%; D101)";
        famine_loss_3: f32 = 0.1, [0, 1], "Share lost in its third and later years (1317: −10%; D101)";
        plague_year: u32 = 248, [0, 2000], "Year of the Black Death (historical; 1348)";
        plague_recur_1: u32 = 261, [0, 2000], "First recurrence (historical; 1361)";
        plague_recur_2: u32 = 269, [0, 2000], "Second recurrence (historical; 1369)";
        plague_later: bool = true, [0, 1], "Hatcher's later national outbreaks, 1375-1479, in the Historical calendar (D108)";
        plague_later_first: f32 = 0.10, [0, 0.5], "Mortality of the first later outbreak (1375; D108)";
        plague_later_last: f32 = 0.06, [0, 0.5], "Mortality of the last later outbreak (1479; D108)";
        endemic_mortality: f32 = 0.015, [0, 0.2], "Yearly share of each place's people lost to endemic disease between outbreaks (Hatcher 1977; D114)";
        endemic_from: u32 = 249, [0, 2000], "First year of the endemic era (1349)";
        endemic_to: u32 = 380, [0, 2000], "Last year of the endemic era (1480)";
        plague_recur_share: f32 = 0.3333333, [0, 1], "Mortality of a recurrence as a share of the first";
        plague_mortality: f32 = 0.4, [0, 0.95], "Mean share of each settlement's people a plague kills";
        plague_spread: f32 = 0.1, [0, 0.5], "Half-width of the uniform spread of plague mortality between settlements";
        plague_years: u32 = 2, [1, 5], "Years over which a plague's deaths are spread";
        silt_from: u32 = 250, [0, 2000], "First year of the window for the historical silting";
        silt_window: u32 = 150, [0, 1000], "Length of that window; the year is drawn within it";
        silt_cost: f32 = 3.0, [0, 50], "Extra cost of each land-water step at a silted harbour";
        silt_depth_m: f32 = 10.0, [0, 200], "Sea no deeper than this counts as a shallow coast";
        silt_interval: f32 = 200.0, [1, 5000], "Mean years between siltings (random mode)";
        resettle_years: u32 = 30, [0, 200], "Years after a plague during which people leave poor land for vacant good land (D61)";
        resettle_rate: f32 = 0.02, [0, 0.2], "Yearly share of a poor-land place's people who resettle";
        plant_interval: u32 = 50, [0, 500], "Years between planted towns, with events on (0 = none) (D63)";
        plant_from: u32 = 150, [0, 2000], "No town is planted before this year";
        plant_pop: f32 = 200.0, [10, 5000], "People a planted town starts with";
        plant_market: f32 = 5.0, [0, 50], "A planted town's starting market size, in multiples of market_w0";
        plant_spacing_km: f32 = 10.7, [0, 50], "Least distance from a planted town to any place over 500 (Bracton's 6 2/3 miles)";
        ship_gain: f32 = 0.3, [0, 0.9], "Share by which sea cost per km falls as ships improve (historical; v5, D65)";
        ship_from: u32 = 200, [0, 2000], "Year ships start to improve (historical; 1300)";
        ship_to: u32 = 400, [0, 2000], "Year the improvement is complete (historical; 1500)";
        famine_interval: f32 = 120.0, [1, 2000], "Mean years between famines (random mode)";
        plague_interval: f32 = 150.0, [1, 2000], "Mean years between plagues (random mode)";
    }
    "Harvests" => {
        local_harvests: bool = true, [0, 1], "Each cell's yield varies year to year with a smooth local field (v6 W1)";
        harvest_sd: f32 = 0.15, [0, 0.5], "Standard deviation of the yearly yield multiplier at the field's nodes";
        harvest_km: f32 = 15.0, [1, 200], "Spacing of the field's nodes, km";
        harvest_min: f32 = 0.5, [0, 1], "Least yearly yield multiplier";
        harvest_max: f32 = 1.5, [1, 3], "Greatest yearly yield multiplier";
        harvest_years: u32 = 500, [0, 5000], "Years of weather drawn at the start of a run, so a seed's weather is fixed (D75)";
    }
    "Mills" => {
        mills: bool = true, [0, 1], "A settlement with a stream or river among its fields has a mill (D56)";
        mill_area: f32 = 20.0, [0, 10000], "Least drainage, km2, of a field's stream for a mill";
        mill_bonus: f32 = 0.03, [0, 0.5], "Yield gained by a settlement with a mill, as a share";
    }
    "View" => {
        density_per_cell: f32 = 1500.0, [100, 20000], "People per built-up cell in the drawn footprint (rendering only)";
        ruin_years: u32 = 60, [0, 255], "Years an abandoned built-up cell stays visible as a ruin";
    }
    "Roads" => {
        roads_enabled: bool = true, [0, 1], "Run the roads system";
        road_interval: u32 = 10, [1, 50], "Years between road building steps";
        road_pairs: u32 = 20, [0, 100], "Trading pairs given a road per step (without traffic_roads)";
        traffic_roads: bool = true, [0, 1], "Roads follow traffic: road cells carry a condition that fades where traffic falls (D54)";
        road_keep_load: f32 = 600.0, [0, 10000], "Trade value a year crossing a road cell below which it decays";
        road_decay: f32 = 0.05, [0, 1], "Condition a little-used road cell loses each road step";
        traffic_bridges: bool = true, [0, 1], "A busy road crossing of a river or stream becomes a bridge (D55)";
        bridge_load: f32 = 1600.0, [0, 100000], "Trade value a year over a road crossing that counts towards a bridge";
        bridge_steps: u32 = 3, [1, 20], "Road steps in a row at bridge_load before a bridge is built";
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
