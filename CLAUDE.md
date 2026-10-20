# Hinterlands — working rules

Read `docs/foundation.md` (the brief and acceptance test) and `docs/decisions.md` (departures from it) before changing anything.

## Commands
- `just test` — `cargo test` (unit + golden determinism test)
- `just validate` — `cargo run --release -p hinterlands-cli -- validate --seeds 20`
- `just run` — headless run with metrics every 50 years
- `just web` — `trunk build` of `hinterlands-web`

## Hard rules
- `hinterlands-core` has no I/O and no wasm deps. Viewer and CLI only read `Snapshot`.
- Determinism: one `rand_chacha` Rng per run; no `HashMap`/`HashSet` iteration in core (use `BTreeMap` or sorted `Vec`); no wall-clock; transcendental maths via `libm`.
- Each system is a free function that writes only its own outputs and has an `enabled` flag in `Params`.
- No cell size or map size is hard-coded; costs are per km, scaled by `cell_km`.
- Nothing marked Later in the foundation is built before v1 passes validation.
- No hosted CI. A milestone is done only when `just test` and `just validate` pass locally; record it in `docs/milestones.md` with numbers, seeds, params and the golden hash.
- Any departure from the spec goes in `docs/decisions.md`.
- Commit messages name the milestone and system, e.g. `M2 cultivation: release hysteresis`.
