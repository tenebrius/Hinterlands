//! Headless runner: runs, validation, renders, benchmarks and ground-truth scoring.

mod png;
mod probe;
mod render;
mod sweep;
mod truth;
mod validate;

use hinterlands_core::metrics;
use hinterlands_core::{Params, Sim};
use hinterlands_mapgen::{generate, MapParams};
use std::collections::BTreeMap;
use std::time::Instant;

const USAGE: &str = "\
hinterlands <command> [options]

commands:
  run       --map-seed N --seed N --years 500 [--params p.json] [--every 50]
            [--png out.png] [--snapshot out.bin] [--truth truth.csv] [--towns out.csv] [--cells out.csv] [--dem file.asc]
  validate  [--maps 3] [--seeds 20] [--years 500] [--params p.json] [--threads N]
  render    --map-seed N --png out.png [--seed N --years N] [--layer terrain|yield|forest|score|cost]
  bench     [--map-seed N] [--seed N] [--years 500]
  hash      [--map-seed N] [--seed N] [--years 500]   print the final snapshot hash
  sweep     --set name=a,b [--set name2=c,d] [--seeds 2] [--maps 1] [--years 500] [--params p.json]
  search    --range name=lo:hi [...] [--n 40] [--seeds 1] [--maps 1] [--best-out p.json]
  params    print the parameter table as Markdown
";

pub struct Args {
    map: BTreeMap<String, String>,
}

impl Args {
    fn parse(items: &[String]) -> Self {
        let mut map = BTreeMap::new();
        let mut it = items.iter();
        while let Some(k) = it.next() {
            if let Some(k) = k.strip_prefix("--") {
                let v = it.next().cloned().unwrap_or_default();
                map.insert(k.to_string(), v);
            }
        }
        Self { map }
    }
    pub fn str(&self, k: &str) -> Option<&str> {
        self.map.get(k).map(|s| s.as_str())
    }
    pub fn u64(&self, k: &str, default: u64) -> u64 {
        self.str(k)
            .map(|v| {
                v.parse()
                    .unwrap_or_else(|_| die(&format!("--{k}: not a number")))
            })
            .unwrap_or(default)
    }
}

pub fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(2)
}

pub fn load_params(args: &Args) -> Params {
    match args.str("params") {
        Some(path) => {
            let text =
                std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
            serde_json::from_str(&text).unwrap_or_else(|e| die(&format!("{path}: {e}")))
        }
        None => Params::default(),
    }
}

pub fn load_map_params(args: &Args) -> MapParams {
    match args.str("map-params") {
        Some(path) => {
            let text =
                std::fs::read_to_string(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
            serde_json::from_str(&text).unwrap_or_else(|e| die(&format!("{path}: {e}")))
        }
        None => MapParams::default(),
    }
}

pub fn make_sim(args: &Args) -> Sim {
    let params = load_params(args);
    let terrain = match args.str("dem") {
        Some(path) => {
            let km = args
                .str("cell-km")
                .map(|v| v.parse().unwrap_or(1.0))
                .unwrap_or(1.0);
            hinterlands_mapgen::import_dem(path, km, &load_map_params(args))
                .unwrap_or_else(|e| die(&e))
        }
        None => generate(args.u64("map-seed", 1), &load_map_params(args)),
    };
    Sim::new(terrain, params, args.u64("seed", 1))
}

fn print_metrics_header() {
    println!(
        "{:>5} {:>5} {:>8} {:>7} {:>5} {:>5} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
        "year",
        "count",
        "total",
        "largest",
        ">2k",
        ">5k",
        "share",
        "slope",
        "space",
        "sited",
        "fail",
        "forest"
    );
}

fn print_metrics(m: &metrics::Metrics) {
    println!(
        "{:>5} {:>5} {:>8.0} {:>7.0} {:>5} {:>5} {:>6.3} {:>6.2} {:>6.1} {:>6.2} {:>6.2} {:>6.2}",
        m.year,
        m.count,
        m.total_pop,
        m.largest,
        m.over_2000,
        m.over_5000,
        m.largest_share,
        m.rank_slope,
        m.spacing_median_km,
        m.siting_share,
        m.failure_rate,
        m.forest_near_cities
    );
}

fn cmd_run(args: &Args) {
    let years = args.u64("years", 500) as u32;
    let every = args.u64("every", 50) as u32;
    let mut sim = make_sim(args);
    let t0 = Instant::now();
    print_metrics_header();
    print_metrics(&metrics::compute(&sim.snapshot(), &sim.terrain));
    let mut founded_log = 0;
    for y in 1..=years {
        sim.step(1);
        let snap = sim.snapshot();
        founded_log += snap.log.len();
        if every > 0 && y % every == 0 {
            print_metrics(&metrics::compute(&snap, &sim.terrain));
        }
    }
    let ms = t0.elapsed().as_secs_f64() * 1000.0 / years.max(1) as f64;
    let snap = sim.snapshot();
    println!(
        "hash {:016x}  {:.2} ms/year  {} log entries  best site score {:.2}",
        snap.hash(),
        ms,
        founded_log,
        snap.tally.last_best_score
    );
    if let Some(mouth) = sim
        .terrain
        .river_mouth
        .filter(|_| args.str("mouth").is_some())
    {
        let t = &sim.terrain;
        println!(
            "main river mouth {:?}, drainage {:.0} km²",
            t.grid.xy(mouth as usize),
            t.flux[mouth as usize]
        );
        let mut near: Vec<_> = snap
            .settlements
            .iter()
            .map(|s| (t.grid.dist_km(s.anchor as usize, mouth as usize), s))
            .filter(|x| x.0 <= 15.0)
            .collect();
        near.sort_by(|a, b| a.0.total_cmp(&b.0));
        for (d, s) in near {
            println!(
                "  #{} at {:?}, {:.1} km: pop {:.0}, through {:.0}, food in {:.0}",
                s.id,
                t.grid.xy(s.anchor as usize),
                d,
                s.pop,
                s.through,
                s.food_in
            );
        }
    }
    if args.str("water").is_some() {
        let t = &sim.terrain;
        let (mut nav, mut stream, mut sea, mut built) = (0, 0, 0, 0);
        for i in 0..t.n() {
            if snap.built[i] != hinterlands_core::state::NONE {
                built += 1;
                if t.is_nav(i) {
                    nav += 1
                }
                if t.water[i] == hinterlands_core::terrain::Water::Stream {
                    stream += 1
                }
                if t.is_sea(i) {
                    sea += 1
                }
            }
        }
        let anchors_nav = snap
            .settlements
            .iter()
            .filter(|s| t.is_nav(s.anchor as usize))
            .count();
        let anchors_stream = snap
            .settlements
            .iter()
            .filter(|s| t.water[s.anchor as usize] == hinterlands_core::terrain::Water::Stream)
            .count();
        println!("built cells {built}: on navigable river {nav}, on stream {stream}, on sea {sea}; anchors on navigable river {anchors_nav}, on stream {anchors_stream} of {}", snap.settlements.len());
    }
    if args.str("young").is_some() {
        // Settlements founded during the run: size, fields and food ratio by age.
        let mut v: Vec<_> = snap.settlements.iter().filter(|s| !s.initial).collect();
        v.sort_by_key(|s| s.founded);
        let bands = [(0u32, 50u32), (50, 100), (100, 200), (200, 500)];
        for (lo, hi) in bands {
            let g: Vec<_> = v
                .iter()
                .filter(|s| (lo..hi).contains(&(snap.year - s.founded)))
                .collect();
            if g.is_empty() {
                continue;
            }
            let mut pops: Vec<f32> = g.iter().map(|s| s.pop).collect();
            pops.sort_by(|a, b| a.total_cmp(b));
            let nofield = g.iter().filter(|s| s.fields == 0).count();
            let hub = g.iter().filter(|s| snap.hub[s.anchor as usize]).count();
            println!(
                "age {lo:>3}-{hi:<3}: {:>3} alive, median pop {:>6.0}, p90 {:>6.0}, no fields {nofield}, on hub {hub}, mean ratio {:.2}",
                g.len(),
                pops[pops.len() / 2],
                pops[pops.len() * 9 / 10],
                g.iter().map(|s| s.food_ratio).sum::<f32>() / g.len() as f32
            );
        }
    }
    if let Some(n) = args.str("detail") {
        print_detail(&snap, &sim.terrain, n.parse().unwrap_or(10));
    }
    if let Some(path) = args.str("png") {
        let layer = args.str("layer").unwrap_or("terrain");
        let img = render::render(&sim.terrain, Some(&snap), layer, 4, None);
        std::fs::write(path, img).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    }
    if let Some(path) = args.str("snapshot") {
        std::fs::write(path, snap.to_bytes()).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    }
    if let Some(path) = args.str("truth") {
        truth::score(path, &sim.terrain, &snap);
    }
    if let Some(path) = args.str("towns") {
        truth::dump(path, &sim.terrain, &snap);
    }
    if let Some(path) = args.str("cells") {
        truth::dump_cells(path, &sim.terrain);
    }
}

/// The largest settlements' accounts, for diagnosing the model.
fn print_detail(snap: &hinterlands_core::Snapshot, t: &hinterlands_core::Terrain, n: usize) {
    let mut v: Vec<_> = snap.settlements.iter().collect();
    v.sort_by(|a, b| b.pop.total_cmp(&a.pop));
    println!(
        "{:>4} {:>9} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>5} {:>6} {}",
        "id", "xy", "pop", "prod", "fd_in", "fd_out", "trade", "thru", "ratio", "fields", "site"
    );
    let show = |s: &&hinterlands_core::snapshot::SettlementView| {
        let a = s.anchor as usize;
        let site = format!(
            "{}{}{}",
            if t.touches(a, |j| t.is_nav(j)) {
                "R"
            } else {
                ""
            },
            if t.is_coastal(a) { "C" } else { "" },
            if snap.hub[a] { "H" } else { "" }
        );
        println!(
            "{:>4} {:<14} {:>9} {:>7.0} {:>7.0} {:>7.0} {:>7.0} {:>7.0} {:>7.0} {:>5.2} {:>6} {}",
            s.id,
            s.name,
            format!("{:?}", t.grid.xy(a)),
            s.pop,
            s.food_prod,
            s.food_in,
            s.food_out,
            s.trade_in,
            s.through,
            s.food_ratio,
            s.fields,
            site
        );
    };
    v.iter().take(n).for_each(show);
    if v.len() > n {
        println!("  ...");
        v.iter().rev().take(3).for_each(show);
    }
}

fn cmd_render(args: &Args) {
    let path = args.str("png").unwrap_or_else(|| die("--png required"));
    let years = args.u64("years", 0) as u32;
    let mut sim = make_sim(args);
    sim.step(years);
    let snap = sim.snapshot();
    let layer = args.str("layer").unwrap_or("terrain");
    let from = args.str("from").map(|v| {
        v.parse::<usize>()
            .unwrap_or_else(|_| die("--from: cell index"))
    });
    // `--bare` draws the terrain alone, without settlements, fields or roads.
    let shown = args.str("bare").is_none().then_some(&snap);
    let img = render::render(&sim.terrain, shown, layer, 4, from);
    std::fs::write(path, img).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let t = &sim.terrain;
    let land = (0..t.n()).filter(|&i| t.is_land(i)).count();
    let nav = (0..t.n()).filter(|&i| t.is_nav(i)).count();
    let stream = (0..t.n()).filter(|&i| t.is_river(i)).count() - nav;
    let fords = t.ford.iter().filter(|&&f| f).count();
    // Land cells with a water supply for founding, under the v1 rule and the
    // brook-scale rule (D29).
    let eligible = |spring: bool| {
        let q = Params {
            spring_water: spring,
            ..sim.params.clone()
        };
        let k = (0..t.n())
            .filter(|&i| {
                t.is_land(i) && hinterlands_core::systems::founding::has_water_supply(t, &q, i)
            })
            .count();
        100.0 * k as f32 / land.max(1) as f32
    };
    println!(
        "founding water supply: {:.0}% of land (v1 rule), {:.0}% (brook rule)",
        eligible(false),
        eligible(true)
    );
    let harb = t.harbour.iter().filter(|&&f| f).count();
    let mean_yield = (0..t.n())
        .filter(|&i| t.is_land(i))
        .map(|i| t.food_yield[i])
        .sum::<f32>()
        / land as f32;
    for f in &t.features {
        println!(
            "  {:?} {:<28} at {:?}, size {:.0}",
            f.kind,
            f.name,
            t.grid.xy(f.label_cell as usize),
            f.size
        );
    }
    println!(
        "land {land} cells, navigable {nav}, stream {stream}, fords {fords}, harbours {harb}, mean yield {mean_yield:.2}, river mouth {:?}",
        t.river_mouth.map(|c| t.grid.xy(c as usize))
    );
}

fn cmd_bench(args: &Args) {
    use hinterlands_core::{reach, systems};
    let years = args.u64("years", 500) as u32;
    let mut sim = make_sim(args);
    let names = [
        "refresh",
        "cultivation",
        "trade",
        "growth",
        "founding",
        "failure",
        "roads",
    ];
    let mut spent = [0f64; 7];
    let mut worst = 0f64;
    let t0 = Instant::now();
    for _ in 0..years {
        let ty = Instant::now();
        let (st, t, p, rng) = (&mut sim.state, &sim.terrain, &sim.params, &mut sim.rng);
        // Mirrors `step_year`, timing each phase.
        st.log.clear();
        let mut lap = |k: usize, f: &mut dyn FnMut()| {
            let s = Instant::now();
            f();
            spent[k] += s.elapsed().as_secs_f64() * 1000.0;
        };
        lap(0, &mut || reach::refresh_due(st, t, p));
        lap(1, &mut || systems::cultivation::run(st, t, p, rng));
        lap(2, &mut || systems::trade::run(st, t, p, rng));
        lap(3, &mut || systems::growth::run(st, t, p, rng));
        lap(4, &mut || systems::founding::run(st, t, p, rng));
        lap(5, &mut || systems::failure::run(st, t, p, rng));
        lap(6, &mut || systems::roads::run(st, t, p, rng));
        st.year += 1;
        worst = worst.max(ty.elapsed().as_secs_f64() * 1000.0);
    }
    let mean = t0.elapsed().as_secs_f64() * 1000.0 / years as f64;
    println!(
        "{} settlements at year {}: mean {:.2} ms/year, worst {:.2} ms",
        sim.state.settlements.len(),
        years,
        mean,
        worst
    );
    for (k, name) in names.iter().enumerate() {
        println!("  {:<12} {:>8.2} ms/year", name, spent[k] / years as f64);
    }
}

fn cmd_hash(args: &Args) {
    let years = args.u64("years", 500) as u32;
    let mut sim = make_sim(args);
    sim.step(years);
    println!("{:016x}", sim.snapshot().hash());
}

fn cmd_params() {
    println!("| Group | Parameter | Default | Range | Meaning |\n| --- | --- | --- | --- | --- |");
    let p = Params::default();
    for info in Params::INFO {
        let v = p.get(info.name).unwrap();
        let shown = if info.boolean {
            (v > 0.5).to_string()
        } else {
            format!("{v}")
        };
        println!(
            "| {} | `{}` | {} | {}–{} | {} |",
            info.group, info.name, shown, info.min, info.max, info.doc
        );
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = argv.first() else {
        print!("{USAGE}");
        return;
    };
    let args = Args::parse(&argv[1..]);
    match cmd.as_str() {
        "run" => cmd_run(&args),
        "validate" => validate::cmd_validate(&args),
        "sweep" => sweep::cmd_sweep(&argv[1..], &args),
        "search" => sweep::cmd_search(&argv[1..], &args),
        "render" => cmd_render(&args),
        "probe" => probe::cmd_probe(&args),
        "failures" => probe::cmd_failures(&args),
        "goods" => probe::cmd_goods(&args),
        "roads" => probe::cmd_roads(&args),
        "bench" => cmd_bench(&args),
        "hash" => cmd_hash(&args),
        "params" => cmd_params(),
        _ => {
            print!("{USAGE}");
            std::process::exit(2);
        }
    }
}
