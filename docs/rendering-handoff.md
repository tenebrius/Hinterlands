# Rendering handoff: the real-scale overhaul

For the graphics session picking this work up after a context reset (written 23 October 2026). The full record is in `docs/rendering.md`, entries R9–R11, together with the section "Next: the real-scale overhaul". Read those, and `CLAUDE.md`, before changing anything.

## Where things stand

- **Branch:** `claude/elegant-ride-g0sr1h`.
  - Last commit: `c3f1922` ("R11 towns revised"), plus this handoff and a small change to `tools/screenshot.mjs`.
  - The branch contains `origin/main` as of 22 October (`e7c0344`).
- **Done:**
  - **R9, stage 1:** the scale framework (zoom to 1 m per point) and roads and rivers at true width.
  - **R10, stage 2:** fields, woods, pasture and footpaths at real scale.
  - **R11, stage 3:** villages and towns laid out from their roads. It was revised after the owner's review: the town's edge now comes from the plots, cities have tenements and courts, and cities are more compact.
- **Done since (23 October), on the owner's notes that true scale was accurate but not fun:**
  - **R12, map style:** a "Map style" panel (`look.rs`, `panels/look.rs`) with road, river, tree and building scales and people per house. It has True scale and Storybook presets and takes `?look=`.
  - **R13, layout:** villages and towns laid out after Watabou's generators. Village houses vary, with farmsteads and gaps, no fences, groves and open greens. Town cores are built up behind, and lanes curve and join.
  - **R14, ink outlines:** a sixth slider that draws dark edges round roofs, crowns, rivers, roads and walls.
  - **R15, dense cores:** town cores packed with buildings that have no street frontage, by aggregation from the fronts (`infill`).
  - **R16, free outlines:** infill houses are four-cornered outlines grown into their gaps (`QHOUSE`, `Shape::q`), drawn with hipped roofs. The shape texture is four texels per shape.
  - **R17, longer outlines:** infill houses are held to at least 1.6 times as long as wide.
  - **R18, no sharp corners:** every infill corner within 25° of square.
  - **R19 (work in progress, abandoned):** block-built cores after Watabou's TownGeneratorOS; the owner found them like stacked bricks.
  - **R20, back to R15's buildings:** R16–R19's buildings and layouts removed by the owner's decision; kept: roads keep to the plan's lines in town, and no groves in densely built ground.
  - **R21, building hierarchy:** each lot has a standing (main streets by the market high, lanes and margins low) that sets house size, height, depth, roof; merchants' houses and courtyard houses in towns, cottages at the edge, a manor in villages.
  - **R22:** less infill, minimum building widths, density falling faster from the centre, and the market place as an irregular hexagon paved by the road drawing, lined with plots (shape texture now five texels per shape).
  - **R23:** connecting lanes (places off the network, dead ends) in `ways.rs`; market street and lane/alley widths; trees kept off roads (roads found once per pixel before the land, `ways_at`); roads lose their edges inside the market place; town plots grow back to fill their blocks; plot ground without bands or stripes.
  - **R24:** one smooth multi-scale bend field (`ways.rs`, `warp`) moves every line on the CPU (the shader's wander is retired); street-front plots turned a few degrees off square, shared by neighbours; town lanes end at their last fronted point or join a nearby street.
  - **R25:** wall ring laid before the lanes (`wall_ring`), lanes kept off it; thicker continuous wall with outer battlements; market stalls; label halo and close-in cartouche; suburbs along main roads to 1.6 x reach.
  - **R26 (owner's test):** plot and court ground not drawn; the land beneath shows round the buildings.
  - **R27:** trees, bushes and shadows kept or left out whole (`tree_clear`, `grove_at`, `crown_top` taking the pixel); no hedges, highway hedges or thorn inside a place's edge. Frame time about doubled (12 s in SwiftShader at 1 m).
- **Not doing next:** stage 4 (buildings to size, traffic pacing, tonnes readouts, label placer, chronicle markers, price surface). The owner has put it aside.
- **Next:** the owner's review of R27; then, if approved, the reviewer's other proposals (roofs by period and clusters, wall styling, street widths by rank, market off the bridgehead, density falloff and ribbon suburbs, label halo).
- **Queued** (in `docs/rendering.md`):
  - **ground inside town walls** (owner, 23 October): less green than outside, mostly compacted soil and paving, with some grass;
  - varied buildings and districts within towns (after stage 4, unless the owner says otherwise);
  - an optimisation stage, which the owner has deferred so that the results are right first.

## Rules that hold
- **Frontend only:** `crates/hinterlands-web`. No change to core, mapgen or CLI.
- **Checks:** the golden hash must stay `65aee272c30de1f9`, and `?selftest=500` must give the same.
- **Engine:** stay on eframe/glow (WebGL2), with the dense detail in the relief shader.
- **No messages:** do not message the implementer or designer. The whole overhaul is handed over as one batch, only after the current development on `main` stops (the owner's decision).
- **Records:** record each step in `docs/rendering.md` as the next R number (R12, …). Commit with a message naming it, and push to the branch.
- **Optimisation is deferred.** Don't spend effort on frame time unless asked, but report it.

## Code map (`crates/hinterlands-web/src/`)
- **`scale.rs`:**
  - the zoom limit `max_zoom(cell_km)` (1 m per point);
  - `symbol_zoom`, which caps symbol sizes at the old 64 points per cell;
  - `symbol_alpha`, which fades map symbols between 64 and 256 points per cell unless the layer "Map symbols at full zoom" (id `symbols`) is on;
  - the scale bar.
- **`layers/relief.rs` + `relief.frag`:** the relief shader.
  - **Textures:** land, cover, plot, town, ways (×3), ground, and town shapes (×4).
  - **Splicing:** `relief.frag` takes in `country.frag` and `town.frag` at their `// @include` markers.
  - **Per pixel in `main()`:** `land_color` (ground, pasture, fields, woods, ruins, seasons), then `town_ground`, then `ways_color` (rivers, roads, paths, town streets), then building shadows and `town_buildings`, then hillshade, then the sea and the coast.
- **`layers/ways.rs`:**
  - **Lines:** river and road centrelines (4 rounds of Chaikin smoothing), kept off the rivers by `Banks::push`, plus footpaths and the town plan's streets.
  - **Data:** a segment texture with a 250 m index.
  - **Segment kinds:** 0 stream, 1 navigable river, 2 track, 3 road, 4 footpath, 5 town street.
  - **Widths:** rivers 0.6 √A m; roads 3–8 m by traffic (`routes::busy`).
  - It also calls the town planner and returns `(WayData, ShapeData, Plan)`.
- **`layers/town.rs`:** the town planner, on the CPU, rebuilt yearly (every 5 years while playing).
  - **`lay_out`**, per place, largest first:
    - main streets from the roads (or tracks; a high street if neither reaches);
    - a market place, or a green;
    - lane generations, plus back lanes;
    - churches, a cathedral, friaries;
    - plots via `place_plots`, with the house, a barn or back range, tenements and courts;
    - city walls via `wall_line` and `build_wall`.
  - **`boundary`:** each place's radial boundary, used to trim its lanes and for the area figures.
  - **`layout`:** builds the shape texture, a two-level index (cell, then 62.5 m squares), and per square the built share (`.b`) and the town-land distance field (`.a`).
  - **`Plan::summary`:** logs households, houses and area against `expected_ha` at info level. Switch it to `warn` temporarily to see it in the browser console.
- **`layers/country.frag`:**
  - parcels (Voronoi, about 170 m), furlongs and strips, meadows, closes, hedges and standard trees;
  - crowns (three jittered grids), coppice and downland;
  - `town_land` and `built_share`, the town edge, and the town's closes (`TOWN_CLOSE`).
- **`layers/town.frag`:** `town_scan` (the shapes per pixel), `town_ground` (plots, courts, market, green, churchyard, precinct, ditch), roofs (thatch, tile, slate), towers, walls and gates.
- **`layers/routes.rs`, `rivers.rs`, `parish.rs`, `dots.rs`, `goods.rs`, `wool.rs`, `outside.rs`, `prices.rs`, `buildings.rs`, `settlements.rs`:** map symbols that fade out close in.

## The look (R12–R14)
- **`Look`** (`look.rs`) is held in `ViewState::look`, set by `Msg::SetLook` or `?look=`.
- **CPU side:**
  - `ways.rs` multiplies road, track, street, footpath and river widths.
  - `town.rs` multiplies every size by `z` (the building scale). Lots deepen by √z, and open places and great buildings grow with z^0.6.
  - A changed look lays roads and towns out again (`ReliefLayer::ways_look`).
- **Shader side:**
  - `u_look` (roads, rivers, trees, buildings) and `u_ink`.
  - While a road, a tree or a building is drawn, `g_unit` holds its scale, and `mpx()` divides by it, so its detail is in its own metres.
  - `g_bnear` (from the town scan, which now runs before the land) keeps trees off the houses.

## How to build and look
- **Build:**

  ```
  cd crates/hinterlands-web
  trunk build --release --dist <dir>
  ```

  The web crate does not build natively (winit); check it with `cargo check --target wasm32-unknown-unknown`.
- **Serve** with `python3 -m http.server <port> --directory <dir>`.
- **Screenshots:**

  ```
  node tools/screenshot.mjs "http://localhost:<port>/?year=450&season=1&off=legend,hovercard,selection&at=X,Y,Z" out.png move:150,850
  ```

  - `at=X,Y,Z` is the map point in cells and points per cell (1000 is 1 m per point on maps with 1 km cells).
  - Full-zoom shots of large towns take up to a minute in SwiftShader.
- **Repetition check:**

  ```
  python3 -I tools/rhythm.py --before A.png --after B.png
  ```

  It fails when the spectral peak passes 30 and is 1.5 times the before shot's.
- **Clippy:** the baseline is 41 warnings. Keep it there.
- **Formatting:** `cargo fmt -p hinterlands-web` reformats `layers/prices.rs`, `panels/chronicle.rs` and `panels/inspector.rs`, which were not formatted before. Revert those three files after formatting, keeping only your own changes.
- **Hash checks:** `just test`; `cargo run --release -p hinterlands-cli -- hash --years 500`; and in the browser `?selftest=500`.

## Places worth looking at (map 1, seed 1)
- **Asherfield** (31.5, 15.5):
  - year 450: a street village of about 475 on a busy highway;
  - year 200: a walled city of 4,500.
- **Dewermouth** (16.6, 63.4): the port city of 15,000 at the river mouth, at year 450.
- **Liningbourne** (74.5, 36.5): a walled upland city of 5,900, at year 450.
- **Mereelwick** (46.5, 106.5): a river town with no road.
- **Downland** (64.5, 23.5): pasture with dew ponds and sheep folds.

## Known issues
- **Dewermouth:**
  - It houses only 2,243 of its 3,342 households.
  - It keeps two longer arms along its north and north-east roads.
- **Other shortfalls:** Aldford houses about 70% and Mereelwick about 58%.
- **Frame time:** at full zoom over a large town, up to about 50 s a frame in SwiftShader. The cost is the town shape scan, the parcels and the tree grids.
- **Stage-4 leftovers:**
  - any road over water is drawn as a bridge (not only the model's bridges);
  - mills are still glyphs;
  - road widths follow traffic only (road condition is not in the snapshot).
