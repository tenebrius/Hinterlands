# Rendering overhaul

Work on the viewer's look, done on the branch `claude/elegant-ride-g0sr1h` while engine work continues on `main`. Everything here is a view layer under the rule in the foundation (*if it can be recomputed from the state at any year, it is a view layer*): no change to `hinterlands-core`, `hinterlands-mapgen` or the CLI, and the golden hash is unchanged.

The entries below are numbered R1, R2… so they do not take decision numbers on `main`. R1 and R2 were merged into `main` as D36 and D37, and R3 as D42; later steps get the next free D numbers when they merge.

## Handover: merging into `main`

This section describes the step waiting to be merged; it is rewritten for each step.

- **Step: R3, the farmed countryside in the shader.** Branch `claude/elegant-ride-g0sr1h`, started from `main` at `7be0f48`.
- **Files touched.** All in `crates/hinterlands-web/src/layers/`:
  - `relief.rs` and `relief.frag`: two per-year textures (land use; plots) and the countryside and season drawing.
  - `countryside.rs`: the strip-direction rule is now the shared function `strip_orientation`, and the strips and pasture tint are skipped while the shader draws them.
  - `fields.rs`, `history.rs` and `seasons.rs`: each skips its per-cell drawing while the shader draws it.
  - Docs and screenshots: this file and `docs/screenshots/overhaul/4-*`, `5-*`.
- **What to check after merging.** `just test`: the golden hash stays `2b00434da9781880`. `just web`, then `?selftest=500` in the browser: the hash must match.
- **No new dependencies.**
- **The old view is one URL away.** `?on=terrain&off=relief,rivers` brings back every per-cell layer. If the shader cannot compile, the same happens by itself and the error is logged.

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

## R3 — Fields, pasture, hedges and seasons drawn in the shader

The 1.x countryside layers drew one square per cell: strips, the pasture tint, hedges along cell edges, ghost strips, and the season tints. On the smooth base these squares were the most visibly blocky thing on the map. The relief shader now draws them per pixel from two more per-year textures:
- **Land use:** forest, cultivated, pasture, and the fade of abandoned fields.
- **Plots:** the owner (hashed), strip direction and fallow rotation, whether the field is hedged, and flood meadow.

The rules for which cells are what are unchanged from the per-cell layers:
- strips run away from the nearest road or river (`strip_orientation`, now shared);
- one strip in three lies fallow, moving on each year;
- pasture is open land beside fields with less than 35% forest;
- hedges come after 50 years of farming;
- abandoned fields fade over 200 years;
- the flood meadow and frost rules are the same as before.

What changes is the drawing:
- **Field outlines.** Each block of farmed cells gets one smooth outline, roughened with noise. Strips lie in furlongs whose boundaries wander slightly, and each strip has a faint reversed S (the curve of a plough team turning). Far out a field is a plain tint; close in, the strips appear.
- **Hedges.** They follow the field outline, and the boundary between neighbouring holdings, with varying thickness.
- **Order.** Pasture is drawn under the fields, and fields under the forest, so a field still being cleared keeps trees standing on it.
- **Seasons.** Spring water on the flood meadows, autumn colour in the woods, and winter frost and snow are applied per pixel.
- **Layer toggles.** Each part is drawn only while its own layer is on (fields, strips, pasture, hedgerows and old fields, seasons). The per-cell layers stand down only while the shader is running; otherwise they draw exactly as before.
- **Still drawn by egui.** Sheep, coppice stools and clearing stumps are point symbols and stay in egui.

Kind: rendering.

## Screenshots

Map 1, seed 1, year 450. The files are in `docs/screenshots/overhaul/` as before/after pairs:
- R1–R2, summer, on v1.1: `1-overview`, `2-region` (zoomed to the central river valley) and `3-coast` (zoomed to the coast, river mouth and Chedesworth).
- R3, on v1.2: `4-fields` (zoomed to Chedesworth and Harnfield in summer) and `5-winter` (the central valley in winter).

## Next steps, in order

1. Roads and water routes smoothed with Chaikin, with an outline under the fill. The trade dots follow the smoothed line. These are now the most visibly blocky thing on the map.
2. Settlement footprints traced as one outline, not per-cell blocks.
3. Lettering: a bundled serif font and halos behind labels.
4. A panel theme to match the map.
