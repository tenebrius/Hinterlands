# Hinterlands build targets. No hosted CI: these are the checks.

default:
    @just --list

# Unit tests, determinism and the golden snapshot hash
test:
    cargo test --release

# The full validation table: 3 maps x 20 seeds x 500 years
validate *ARGS:
    cargo run --release -p hinterlands-cli -- validate --maps 3 --seeds 20 {{ARGS}}

# One headless run with metrics every 50 years
run *ARGS:
    cargo run --release -p hinterlands-cli -- run --map-seed 1 --seed 1 --years 500 {{ARGS}}

# Build the browser app into dist/
web:
    cd crates/hinterlands-web && trunk build --release --dist ../../dist

# Serve the browser app locally
serve: web
    python3 -m http.server 8080 --directory dist

# Timing per simulated year (native)
bench *ARGS:
    cargo run --release -p hinterlands-cli -- bench {{ARGS}}

# Re-record the golden hash after an intended model change
golden:
    cargo run --release -p hinterlands-cli -- hash --map-seed 1 --seed 1 --years 500 > crates/hinterlands-cli/tests/golden.txt

# Real-terrain test: 20 seeds on Kent (uniform soil with BGS chalk and marsh, Roman roads and bridges), scored against the towns of c.1300
real:
    tools/real/run_seeds.sh data/real/kent 20 500 "" "" data/real/kent/map-geology.json
    python3 -I tools/real/score.py data/real/kent target/real/kent docs/real/kent
    python3 -I tools/real/score.py data/real/kent target/real/kent docs/real/kent/interior 10
    rm -f docs/real/kent/interior/*.png docs/real/kent/interior/truth.csv
