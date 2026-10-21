# Rendering overhaul

Work on the viewer's look, done on the branch `claude/elegant-ride-g0sr1h` while engine work continues on `main`. Everything here is a view layer under the rule in the foundation (*if it can be recomputed from the state at any year, it is a view layer*): no change to `hinterlands-core`, `hinterlands-mapgen` or the CLI, and the golden hash is unchanged.

The entries below are numbered R1, R2… so they do not take decision numbers on `main`. R1 and R2 were merged into `main` as D36 and D37; later steps get the next free D numbers when they merge.

## Handover: merging into `main`

- **Files touched.**
  - New: `crates/hinterlands-web/src/layers/relief.rs`, `relief.frag` and `rivers.rs`.
  - Changed: `layers/mod.rs` (two new layers registered), `layers/terrain.rs` (off by default, renamed "(cells)") and `layers/seasons.rs` (spring high water does not tint river cells while the river lines are on).
  - Docs and screenshots: this file, a README line and `docs/screenshots/overhaul/`.
- **What to check after merging.** `just test`: the golden hash stays `68beae826cf800cd`. `just web`, then `?selftest=500` in the browser: the hash must match. Watch `layers/mod.rs` if `main` has added a layer there (for example for the Roman roads); the conflict is only the list in `all()`.
- **No new dependencies.** `eframe` already re-exports `glow` and `egui_glow`, and `Cargo.lock` is unchanged.
- **The old view is one URL away.** `?on=terrain&off=relief,rivers`. If the shader cannot compile, the relief layer draws the old cell texture itself and logs the error.

## R1 — Smooth relief drawn by a shader

The base map was one pixel per cell, stretched with nearest-neighbour filtering, so the map read as a mosaic of 1 km squares. It is now drawn by a fragment shader through an `egui_glow` paint callback, in the same egui frame as every other layer.

- **Inputs.** Per cell: elevation, signed distance to the coast in cells, yield and hillshade, computed on the CPU when the terrain changes. Per year: the forest cover from the snapshot. Each set goes up as one float texture.
- **Per pixel.** The cells are interpolated bicubically. The coast is the zero line of the distance field, wandering by up to about 0.2 of a cell with noise. The sea is shaded from shallow to deep, with engraved shore lines once zoomed in. Land is tinted by yield and height (upland, rock, snow) and lit by a Horn hillshade from the north-west, exaggerated 12×.
- **Forest.** Far out, forest is a tint in proportion to cover. Closer in, it breaks into clumps whose share of the ground is the cover. Closest in, it resolves into tree crowns, with single trees beyond the wood's edge.
- **Patterns.** Noise comes from an integer hash of map position, so patterns are the same on every machine, stay in place under pan and zoom, and never touch the run's generator.
- **Fallback.** The old `TerrainLayer` stays, off by default. It is the fallback and the "before" for comparison.

Kind: rendering.

## R2 — Rivers drawn as lines

Streams and navigable rivers were coloured cells. They are now polylines that follow `flow_dir` from cell centre to cell centre, broken at sources and confluences and ending at the shore. Each line is smoothed with two rounds of Chaikin corner cutting, with endpoints kept so tributaries still meet. Width grows with drainage: `0.012 · flux^0.4` km, capped at 0.45 km and converted to cells by `cell_km`. Navigable stretches are drawn darker. When the seasons layer is on, spring widens every channel by half instead of tinting the cells.

Kind: rendering.

## Screenshots

Map 1, seed 1, year 450, summer. The files are in `docs/screenshots/overhaul/` as before/after pairs: `1-overview`, `2-region` (zoomed to the central river valley) and `3-coast` (zoomed to the coast, river mouth and Chedesworth).

## Next steps, in order

1. Fields, strips and pasture drawn with soft, hedged edges instead of cell squares. These layers are now the most visibly blocky thing on the map.
2. Roads and water routes smoothed with Chaikin, with an outline under the fill. The trade dots follow the smoothed line.
3. Settlement footprints traced as one outline, not per-cell blocks.
4. Lettering: a bundled serif font and halos behind labels.
5. A panel theme to match the map.
