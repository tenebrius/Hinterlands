#!/usr/bin/env bash
# R (D119): validate each setting given as a JSON line on stdin, seeds
# FIRST..FIRST+19, into target/retune/<name>.txt; skips settings already done.
# usage: tools/retune/screen.sh [FIRST_SEED] < settings.txt
cd "$(dirname "$0")/../.."
first=${1:-1}
while read -r v; do
  [ -z "$v" ] && continue
  name=$(echo "$v" | tr -d '{}" ' | tr ':,' '=_')
  [ -z "$name" ] && name=current
  out=target/retune/s${first}_$name.txt
  if [ -f "$out" ] && grep -q "^validation" "$out"; then continue; fi
  echo "$v" > target/retune/s${first}_$name.json
  ./target/release/hinterlands validate --seeds 20 --first-seed "$first" --threads 8 \
    --params target/retune/s${first}_$name.json > "$out" 2>&1
  echo "done $name"
done
