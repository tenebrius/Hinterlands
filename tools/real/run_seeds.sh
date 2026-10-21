#!/usr/bin/env bash
# Run the model on an imported region for seeds 1..N and dump every settlement.
# usage: tools/real/run_seeds.sh data/real/kent 20 [years]
set -euo pipefail
dir=$1; n=$2; years=${3:-500}
out=target/real/$(basename "$dir")
mkdir -p "$out"
cargo build --release -q -p hinterlands-cli
seq 1 "$n" | xargs -P "$(nproc)" -I{} ./target/release/hinterlands run \
    --dem "$dir/$(basename "$dir").asc" --map-params "$dir/map.json" \
    --seed {} --years "$years" --every 1000 --towns "$out/seed{}.csv" --png "$out/seed{}.png" >/dev/null
./target/release/hinterlands render --dem "$dir/$(basename "$dir").asc" --map-params "$dir/map.json" \
    --bare 1 --png "$out/terrain.png" >/dev/null
./target/release/hinterlands run --dem "$dir/$(basename "$dir").asc" --map-params "$dir/map.json" \
    --years 0 --cells "$out/cells.csv" >/dev/null
echo "wrote $n runs to $out"
