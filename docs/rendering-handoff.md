# Rendering handoff: the real-scale overhaul

For the graphics session picking this work up after a context reset (rewritten 23 October 2026, after R27). Read this file, `CLAUDE.md`, and the entries R9–R27 in `docs/rendering.md` (each records what was done and why) before changing anything.

## Where things stand

- **Branch:** `claude/elegant-ride-g0sr1h`. Pull it first.
  - It contains `origin/main` as of 22 October (`e7c0344`).
  - Last step: R29, fewer tree candidates (the second optimisation step); R30 and R31 were tried and reverted.
- **Done:**
  - **R9–R11, stages 1–3 of the real-scale overhaul:**
    - zoom to 1 m per point;
    - roads and rivers at true width;
    - fields, woods and pasture;
    - towns laid out from their roads.
  - **R12–R27, the owner's notes that true scale was accurate but not fun:**
    - **R12, map style:** a "Map style" panel (`look.rs`, `panels/look.rs`).
      - **Sliders:** road, river, tree and building scale, people per house, and ink (R14).
      - **Presets:** True scale and Storybook; `?look=`.
    - **R13, layout:** villages and towns after Watabou's generators.
    - **R14:** ink outlines.
    - **R15:** dense town cores (`infill`).
    - **R16–R19:** free-outline buildings and block-built cores. The owner rejected these, and **R20** restored R15's buildings.
    - **R21, building hierarchy:** each lot has a standing. That gives merchants' houses, courtyard houses, cottages at the edge, and a manor in villages.
    - **R22:**
      - less infill;
      - minimum widths;
      - density falling from the centre;
      - the market place as an irregular hexagon paved by the road drawing.
    - **R23:**
      - connecting lanes (`ways.rs`, `connectors`);
      - a market street, and lanes and alleys by rank;
      - trees off roads;
      - roads lose their edges inside the market place;
      - plots grow back to fill their blocks.
    - **R24:**
      - one multi-scale bend field moves every road, river and path on the CPU (`warp`);
      - plots a few degrees off square;
      - town lanes end at their last house or join a street.
    - **R25:**
      - city walls laid before the lanes (`wall_ring`), with gates on the main roads;
      - a thicker wall;
      - market stalls;
      - a label halo and a close-in cartouche;
      - suburbs along the approach roads.
    - **R26:** plot and court ground is not drawn (the owner's test, kept).
    - **R27:** trees, bushes and shadows are kept or left out whole, and there are no hedges inside a place's edge.
- **R28, optimisation 1:** the rendered map is cached in a framebuffer and redrawn only when the camera, year, season, layers or look change. A still frame takes 50–100 ms in SwiftShader instead of 2.8–16 s. `?redraw=1` turns the cache off, to time the shader.
- **R29, optimisation 2:** the tree search visits only the grid cells that can reach the pixel, colours only the tallest crown, asks clearance last and stops the shadow search at full shadow. Redraws about 42% faster in woods, 20–30% in towns.
- **R30, R31 (tried, reverted):** shorter town shape lists (sub-squares) and fewer grove-density lookups. Both were exact but gained nothing measurable, so the code is R29's. In a town, the cost is the trees' crowns, shadows and clearance themselves.
- **Now: optimisation, continued (the owner's instruction, 23 October).** The frames while moving are still full redraws. See "Optimisation" below.
- **After that, waiting for the owner:**
  - **The reviewer's remaining proposals:**
    - roofs by period, wealth and clusters;
    - the market off the bridgehead;
    - a ditch and bank round towns too small for walls.
  - **Queued:**
    - **Ground inside town walls** (owner, 23 October): less green than outside, mostly compacted soil and paving, with some grass.
    - Varied buildings and districts within towns.
    - Bending the far-out map symbols as the close-in roads are bent (R24 left them straight).
- **Not doing:** stage 4 (buildings to size, traffic pacing, tonnes readouts, label placer, chronicle markers, price surface). The owner has put it aside.

## Rules that hold
- **Frontend only:** `crates/hinterlands-web`. No change to core, mapgen or CLI.
- **Checks:** the golden hash must stay `65aee272c30de1f9` (`just test`, `cargo run --release -p hinterlands-cli -- hash --years 500`), and `?selftest=500` in the browser must give the same.
- **Engine:** stay on eframe/glow (WebGL2), with the detail in the relief shader.
- **Records:** record each step in `docs/rendering.md` as the next R number (R28, …), with screenshots in `docs/screenshots/overhaul/` (next prefix `31-`). Commit with a message naming it, and push to the branch.
- **Sessions:**
  - **Implementer and designer:** do not message them. The overhaul goes to `main` as one batch only when the owner says so.
  - **Reviewer:** the owner allows working with the reviewer session `session_018DLnDSAggGPAKjRucQdQRd` (a second opinion on how things look). Its proposals are built only once the owner approves them.
- **Asking:** when the owner gives a list, do it in order and ask before starting anything large or uncertain.

## Optimisation (next)

The owner deferred frame time until the look was right. That is now the task.

- **Done: a. the cache (R28).** The full baseline of eight views and the times after it are in R28. Later steps time redraws with `?redraw=1`.
- **Done: part of b/c/d for trees (R29).** R29 lists where a redraw's time goes. SwiftShader rewards fewer loop iterations, not skipping work inside them.
- **Old baseline** (`node tools/frametime.mjs <url>`, headless Chromium with SwiftShader, 1,400 × 900, map 1, year 450, summer; only ratios are meaningful):

  | View (`at=`, look) | ms per frame |
  |---|---|
  | Liningbourne, `74.5,36.5,1000`, storybook | 12,066 |
  | wood by a stream, `40,60.5,1000`, storybook | 13,733 |
  | Liningbourne, true scale, 1 m (R24) | 6,250 |

- **Where the time goes**, per pixel, from reading the shader:
  - **Every frame was redrawn**, even when nothing moved (fixed by R28's cache).
  - **Trees** (`country.frag`: `crowns_at`, `crown_top`, `draw_crowns`):
    - three jittered grids × 9 candidates, searched twice (crown and shadow);
    - since R27 each candidate near the pixel calls `tree_clear`, which may scan up to 160 road segments (`road_sd_at`) and 160 town shapes (`town_sd_at`).
  - **Towns:** `town_scan` loops over up to 160 shapes per pixel, with a shadow test for each building.
  - **Fields:** the parcel tessellation (a 3×3 then 5×5 Voronoi search), done twice for closes, plus hedges.
  - **Roads:** `ways_near` loops over up to 160 segments per pixel.
- **Candidate methods** (decide by measuring):
  - **a.** Cache the rendered map in a framebuffer and redraw it only on change, or redraw tiles.
  - **b.** Precompute on the CPU and pass as textures:
    - distance to roads and to buildings, per tile;
    - each tree's keep or clear decision;
    - the parcel tessellation.
  - **c.** Levels of detail: skip crowns, shadows and hedges whose size is under a pixel or two; shorten loops far out.
  - **d.** Early outs: tighter index squares (the shape index is 62.5 m, the road index 250 m), and break loops once nothing nearer can be found.
- **Rule for every optimisation step:** the picture must not change.
  - **Checking:** compare before and after screenshots of the same views pixel by pixel (PIL `ImageChops.difference`). Only smoke, which animates, may differ; anything else must be explained in the record.
  - **Recording:** record the frame times before and after in the R entry.
- **Plan updates:** the original plan for this stage is in `docs/rendering.md`, R10's "Frame time" notes and the section "Next: the real-scale overhaul"; update them as you go.

## Code map (`crates/hinterlands-web/src/`)
- **`look.rs`, `panels/look.rs`:** the map style. `ViewState::look`, `Msg::SetLook`, `?look=`.
- **`scale.rs`:**
  - the zoom limit (1 m per point);
  - symbol sizes capped at 64 points per cell;
  - map symbols fading between 64 and 256 points per cell;
  - the scale bar.
- **`layers/relief.rs` + `relief.frag`:** the relief shader.
  - **Textures:** land, cover, plot, town, ways ×3, ground, shapes ×4.
  - **Uniforms:** `u_look`, `u_ink`.
  - **Splicing:** `relief.frag` splices in `country.frag` and `town.frag` at their `// @include` markers.
  - **`main()`, in order:**
    1. `town_scan` (sets `g_bnear`);
    2. `ways_at` (sets `g_road_sd`; R23);
    3. `land_color`;
    4. `town_ground`;
    5. `ways_color`;
    6. shadows and `town_buildings`;
    7. hillshade, sea and coast.
  - **Detail units:** `g_unit` puts the detail of a road, tree or building in its own metres (`mpx()` divides by it).
  - **Trees:**
    - `tree_clear`, `road_sd_at` and `town_sd_at` decide trees whole (R27);
    - `grove_at` gives a place's grove density at a tree's centre (`g_grove_on`).
- **`layers/ways.rs`:**
  - **Lines:** river, road and track lines, smoothed, bent (`warp`, R24) and kept off rivers (`Banks`); connecting lanes (`connectors`, R23); footpaths; the town plan's streets.
  - **Data:** the segment texture, with a 250 m index.
  - **Kinds:** 0 stream, 1 river, 2 track, 3 road, 4 path, 5 town street.
- **`layers/town.rs`:** the town planner (CPU, rebuilt yearly; every 5 years while playing). `lay_out`, per place:
  - main streets from the roads (to 1.6 × reach for towns' suburbs);
  - the market (a hexagon, `Shape::q`), the hall and stalls, or a green;
  - the city wall ring (`wall_ring`);
  - lane generations and back lanes, kept off the wall;
  - churches and friaries;
  - plots (`place_plots`):
    - standing (R21);
    - a turn off square (R24);
    - growing back (R23);
  - each lot's buildings (`Lot::build_town` / `build_village`);
  - infill of the core (`infill`);
  - the wall (`build_wall`);
  - lanes trimmed to their last house.
  - **Shape texture** (`layout`): five texels per shape, the last three a market's corners. It also builds the two-level index (cell, then 62.5 m squares), the built share and the town-land field.
- **`layers/country.frag`:**
  - parcels, furlongs, meadows and closes;
  - hedges and standard trees;
  - crowns, coppice and downland;
  - `town_land`, `built_share` and `grove_at`.
- **`layers/town.frag`:**
  - `town_scan`, `town_sd_at`;
  - `town_ground`: market, green, churchyard, precinct, ditch (plots and courts are not drawn, R26);
  - roofs, towers, the wall, gates, stalls.
- **`layers/labels.rs`:** names, with the halo and the close-in cartouche (R25).
- **Other layers** (`routes`, `rivers`, `parish`, `dots`, `goods`, `wool`, `outside`, `prices`, `buildings`, `settlements`): map symbols that fade out close in.

## How to build and look
- **Build:** `cd crates/hinterlands-web && trunk build --release --dist <dir>` (about a minute).
  - **Check only:** `cargo check --target wasm32-unknown-unknown`. The crate does not build natively.
- **Serve:** `python3 -m http.server <port> --directory <dir>` (in the background).
- **Screenshot:**

  ```
  node tools/screenshot.mjs "http://localhost:<port>/?year=450&season=1&off=legend,hovercard,selection&look=storybook&at=X,Y,Z" out.png move:150,850
  ```

  - `at=X,Y,Z`: the map point in cells, and points per cell (1000 is 1 m per point).
  - Full-zoom shots take up to a few minutes in SwiftShader.
  - Run several in parallel with `&` and `wait`.
- **Frame time:** `node tools/frametime.mjs "<same url>"`. Since R28 a still view is cached; add `&redraw=1` to time the shader.
- **Shader errors** show only in the browser console: "relief shader unavailable, drawing cells instead". GLSL reserved words, `patch` among them, have bitten before.
- **Planner logs:** `Plan::summary` logs households and houses at info level. Switch it to `warn` temporarily to see it.
- **Clippy:** `cargo clippy --target wasm32-unknown-unknown`. The web crate has 7 warnings; keep it there.
- **Formatting:** `cargo fmt -p hinterlands-web` also reformats `layers/prices.rs`, `panels/chronicle.rs` and `panels/inspector.rs`. Revert those three with `git checkout` afterwards.
- **Repetition check:** `python3 -I tools/rhythm.py --before A.png --after B.png`.

## Places worth looking at (map 1, seed 1)
- **Asherfield** (31.5, 15.5):
  - year 450: a village of about 475;
  - year 200: a walled city of 4,500, a good view for walls and trees (`at=31.48,15.66,1000`).
- **Liningbourne** (74.5, 36.5): a walled upland city of 5,900, with a bridge and market, year 450.
- **Dewermouth** (16.6, 63.4): the port city of 15,000.
- **Halenford** (44.5, 71.5): a town of about 500.
- **Glenden** (33.4, 62.25): a village joined to the network by R23's lanes.
- **Mereelwick** (46.5, 106.5): a river town with no road.

## Known issues
- **Houses housed:** the cramped port and river towns do not house all their households (Dewermouth about two-thirds).
- **Far-out symbols:** the map symbols are straight while the close-in roads bend, so roads change shape around 15 m per pixel.
- **Frame time:** see Optimisation.
- **Stage-4 leftovers:**
  - any road over water is drawn as a bridge;
  - mills are glyphs;
  - road widths follow traffic only.
