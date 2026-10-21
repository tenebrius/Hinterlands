# Rendering overhaul

Work on the viewer's look, done on the branch `claude/elegant-ride-g0sr1h` while engine work continues on `main`. Everything here is a view layer under the rule in the foundation (*if it can be recomputed from the state at any year, it is a view layer*): no change to `hinterlands-core`, `hinterlands-mapgen` or the CLI, and the golden hash is unchanged.

The entries below are numbered R1, R2… so they do not take decision numbers on `main`. R1 and R2 were merged into `main` as D36 and D37, R3 as D42, R4 as D43, R5 as D45, R6 as D46 and R7 as D47; later steps get the next free D numbers when they merge.

## Handover: merging into `main`

This section describes what is waiting to be merged; it is rewritten each time.

- **Steps: R6 (footpaths, parish boundaries and quays as smoothed lines) and R7 (lettering and panel theme).** Branch `claude/elegant-ride-g0sr1h`, from `main` at `5b5792f`. Done together in one batch at the owner's request, so they merge together.
- **Files touched.** All in `crates/hinterlands-web/`:
  - R6, in `src/layers/`: `lines.rs` gains `chains` (edges to pin-to-pin chains), and `routes.rs` now uses it with unchanged output. `parish.rs` draws footpaths and boundaries as smoothed dashed lines. `relief.frag` draws sea quays on the smoothed shore, and `relief.rs` gives the town texture's fourth value on built cells the town-or-larger flag. `rivers.rs` embanks rivers through towns, and `buildings.rs` skips the cell-edge quays while the shader runs.
  - R7: new `src/style.rs` (fonts and theme), installed from `app.rs`, registered in `main.rs`. New `assets/fonts/` holds EB Garamond Medium and Medium Italic (Latin subsets, about 50 KB each) and the font's `OFL.txt`. `layers/labels.rs` and the legend in `layers/mod.rs` use the serif. `app.rs` gives panel titles the serif and a new map surround colour.
  - Docs and screenshots: this file, a README line and `docs/screenshots/overhaul/8-*`, `9-*`.
- **New files carry a licence.** The fonts are under the SIL Open Font License 1.1, which allows bundling. `OFL.txt` sits beside them. The wasm grows by about 130 KB (5.09 to 5.23 MB).
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

## R4 — Roads, routes and trade drawn as smoothed lines

Roads and trade routes were straight segments from cell centre to cell centre, so every route zig-zagged along the eight grid directions.

- **Chains.** Each year's route segments are joined into chains, one kind at a time: water, land trade off the roads, and roads.
- **Pins.** Chains break only at pins. A pin is a junction, an end, a cell where a kind of route starts or stops, or a settlement centre, so roads still run through towns.
- **Smoothing.** Each chain is smoothed with the same Chaikin corner cutting as the rivers (now shared in `lines.rs`). Width still follows each segment's flow.
- **Roads.** Once a road is 2.5 px wide it is drawn cased: a dark edge under a light fill, with all edges drawn before all fills so junctions join cleanly. Below that width a road is a single brown line, as before. Quiet roads still fade back.
- **Tracks and water.** Land trade off the roads is a dashed track. Water trade stays a pale line.
- **Trade dots.** Each trade path is smoothed the same way and split at the same pins, so the moving dots run on the drawn lines. Dot positions are cached in map space and placed on screen each frame, so panning and zooming no longer rebuild them.
- **Unchanged.** Which segments are drawn (the 2% share rule), and the trip, cargo and speed rules for dots.

Kind: rendering.

## R5 — Settlement footprints, ruins and city walls drawn in the shader

Built-up areas were drawn per cell: a texture far out, and rounded squares with houses close in. City walls ran along cell edges with towers at the corners. The relief shader now draws them from a per-year texture with four values per cell: built up, ruin, inside a city's walls, and how fresh the ruin is.

- **Footprints.** Each built-up area has one smooth outline, placed by signed distance from a smoothed field and its exact gradient. Far out it is a town-red tint. At middle zoom it is a tan area with an inked edge. Close in it fills with street blocks a third of a cell across, the grid bent slightly: house plots along the lanes, gardens behind. Plots empty out towards the edge of town.
- **Ruins.** A grey patch that fades with the ruin's age, showing broken house walls close in.
- **Walls.** City walls follow the outline of the city's built-up area. Towers sit on the wall line at half-cell spacing, snapped onto the line along the gradient.
- **Order.** Roads and rivers are drawn over the towns, so roads read as main streets. Settlement markers, churches, squares, quays and bridges are still drawn by egui.
- **Toggles.** Footprints follow the settlements toggle and walls follow the buildings toggle. The per-cell versions stand down only while the shader runs.
- **Known difference.** Where a road runs along a river through a town, the road now covers the river's line there, as it already did outside towns. The per-cell view drew the river channel over everything in built-up cells (D22). If this matters, the river line can be redrawn over roads inside built-up areas.

Kind: rendering.

## R6 — Footpaths, parish boundaries and quays drawn as smoothed lines

These were the last pieces of the map drawn along the grid.

- **Footpaths.** Joined into chains with the shared `lines::chains` (now also used by the routes), kept sharp at settlements, smoothed like the roads and drawn dashed.
- **Parish boundaries.** Traced along cell corners, joined into chains that keep their corner where three parishes meet, smoothed with three rounds and drawn as a faint dashed line. Which settlement each cell belongs to is unchanged.
- **Sea quays.** The relief shader draws them as a timber and stone edge on the land side of the smoothed shore, wherever a town or larger meets the sea. The town texture's fourth value carries that flag on built cells; it still holds the ruin's age on ruined ones.
- **River quays.** The river layer embanks rivers where they run through a town or larger, as a brown bank under the water.
- **Stand-down.** The cell-edge quay glyphs stand down while the shader runs. Warehouses are still drawn by egui.

Kind: rendering.

## R7 — Map lettering and panel theme

- **Font.** The viewer bundles EB Garamond (SIL Open Font License 1.1) at medium weight, regular and italic, subset to Latin. Missing characters fall back to the default fonts.
- **Labels.** Map labels use it, italic for seas, bays, rivers and forests, replacing egui's slanted sans. Sizes are scaled to keep the old x-height. The halo is the text traced at eight points, in place of four.
- **Panels.** Headings and panel titles use the serif. The panels take a parchment theme in the map's colours: ink-brown text, tan widgets, a brown selection. The theme applies in both light and dark mode, because the map has one palette. The area around the map is a warm dark grey in place of blue-black.
- **Unchanged.** Panel contents and layout.

Kind: rendering.

## Screenshots

Map 1, seed 1, year 450. The files are in `docs/screenshots/overhaul/` as before/after pairs:
- R1–R2, summer, on v1.1: `1-overview`, `2-region` (zoomed to the central river valley) and `3-coast` (zoomed to the coast, river mouth and Chedesworth).
- R3, on v1.2: `4-fields` (zoomed to Chedesworth and Harnfield in summer) and `5-winter` (the central valley in winter).
- R4, on v2 G2: `6-roads` (the same view as `4-fields`, roads and routes before and after).
- R5, on v2 G3: `7-town` (Langden, a walled city, at full zoom; before is the per-cell view).
- R7: `8-lettering` (the whole map and panels). R6: `9-port` (Wenenford: quays on the smoothed shore, smoothed footpaths).

## Possible next steps

The overhaul's planned steps are done. Ideas from the original proposal not yet built, none of them needed:

1. Style presets (`?style=atlas`, `?style=topo`) that swap the palette and symbols.
2. Fading between years at slow speeds, so forest, fields and footprints change smoothly.
3. The same smooth base map in the CLI's headless PNG renderer (`render.rs`), so milestone screenshots match the browser.
4. Rivers redrawn over roads inside towns (the D45 difference), if it is wanted.
