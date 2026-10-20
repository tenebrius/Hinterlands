//! Headless runner: runs, validation, renders, benchmarks and ground-truth scoring.

mod png;
mod render;
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
            [--png out.png] [--snapshot out.bin] [--truth truth.csv] [--dem file.asc]
  validate  [--maps 3] [--seeds 20] [--years 500] [--params p.json] [--threads N]
  render    --map-seed N --png out.png [--seed N --years N] [--layer terrain|yield|forest|score|cost]
  bench     [--map-seed N] [--seed N] [--years 500]
  hash      [--map-seed N] [--seed N] [--years 500]   print the final snapshot hash
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
            hinterlands_mapgen::import_dem(path, km).unwrap_or_else(|e| die(&e))
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
    let img = render::render(&sim.terrain, Some(&snap), layer, 4, from);
    std::fs::write(path, img).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let t = &sim.terrain;
    let land = (0..t.n()).filter(|&i| t.is_land(i)).count();
    let nav = (0..t.n()).filter(|&i| t.is_nav(i)).count();
    let stream = (0..t.n()).filter(|&i| t.is_river(i)).count() - nav;
    let fords = t.ford.iter().filter(|&&f| f).count();
    let harb = t.harbour.iter().filter(|&&f| f).count();
    let mean_yield = (0..t.n())
        .filter(|&i| t.is_land(i))
        .map(|i| t.food_yield[i])
        .sum::<f32>()
        / land as f32;
    println!(
        "land {land} cells, navigable {nav}, stream {stream}, fords {fords}, harbours {harb}, mean yield {mean_yield:.2}, river mouth {:?}",
        t.river_mouth.map(|c| t.grid.xy(c as usize))
    );
}

fn cmd_bench(args: &Args) {
    let years = args.u64("years", 500) as u32;
    let mut sim = make_sim(args);
    let mut worst = 0f64;
    let t0 = Instant::now();
    for _ in 0..years {
        let t = Instant::now();
        sim.step(1);
        worst = worst.max(t.elapsed().as_secs_f64() * 1000.0);
    }
    let mean = t0.elapsed().as_secs_f64() * 1000.0 / years as f64;
    println!(
        "{} settlements at year {}: mean {:.2} ms/year, worst {:.2} ms",
        sim.state.settlements.len(),
        years,
        mean,
        worst
    );
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
        "render" => cmd_render(&args),
        "bench" => cmd_bench(&args),
        "hash" => cmd_hash(&args),
        "params" => cmd_params(),
        _ => {
            print!("{USAGE}");
            std::process::exit(2);
        }
    }
}
