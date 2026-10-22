# Rendering overhaul

Work on the viewer's look, done on the branch `claude/elegant-ride-g0sr1h` while engine work continues on `main`. Everything here is a view layer under the rule in the foundation (*if it can be recomputed from the state at any year, it is a view layer*): no change to `hinterlands-core`, `hinterlands-mapgen` or the CLI, and the golden hash is unchanged.

The entries below are numbered R1, R2… so they do not take decision numbers on `main`. R1 and R2 were merged into `main` as D36 and D37, R3 as D42, R4 as D43, R5 as D45, R6 as D46 and R7 as D47; later steps get the next free D numbers when they merge.

## Handover: merging into `main`

This section describes what is waiting to be merged; it is rewritten each time.

- **Step: R8 (hover card and a fuller chronicle).** Branch `claude/elegant-ride-g0sr1h`, from `main` at `6e170c3`.
- **Files touched.** All in `crates/hinterlands-web/src/`:
  - new `hover.rs` (the card);
  - new `layers/selection.rs` (the ring round the selected place);
  - `layers/mod.rs` (registers the selection layer and the hover card's toggle);
  - `app.rs` (shows the card at the cursor);
  - `main.rs` (registers `hover`);
  - `view.rs` (`Note`, `NoteKind` and the milestones noticed between years: new `ViewState` fields `notes`, `best_rank`, `leader`, `road_km`);
  - `panels/chronicle.rs` (merged notes, filters, click to fly to a place);
  - `panels/mod.rs` (the chronicle opens by default, under Time);
  - `style.rs` (a short tooltip delay).
- **Also touched.** This file and `docs/screenshots/overhaul/10-*`.
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

## R8 — Hover card and a fuller chronicle

So that someone watching can tell what is happening without opening the inspector.

- **Hover card.** It follows the cursor over the map (after 0.15 s) and lists what lies there, from the top down, for the layers that are on:
  - the settlement: size and rank, growing or shrinking over 50 years, short of food, how much of its food it buys, whether trade passes through, and when it was founded;
  - else the houses of a town, or ruins;
  - the fields and whose they are, since when, whether hedged and whether still being cleared;
  - else pasture, coppice or abandoned fields, with the year last farmed;
  - woodland cover;
  - the ground: kind, height and yield;
  - named features there;
  - a road, and the largest trade passing;
  - the site score, with that overlay on.

  It has its own toggle among the layers, "Hover card", and the inspector still holds the full detail of a pinned cell.
- **Chronicle.** It opens by default, under Time. The viewer now adds three kinds of event the core's log does not hold, noticed between years:
  - a place growing into a village, town or city, noted once at the largest size it reaches;
  - a new largest place, which must lead by 2% so near-ties do not flicker;
  - new roads, with the km built.

  Filters choose Growth, Roads, Foundings and Failures. Hamlets that were founded and failed without ever growing are hidden unless "Hamlets" is ticked; there are hundreds of them, and they buried the story. Clicking an entry selects the place, brings it into view and marks it with a ring.
- **View only.** Nothing feeds back.

Kind: rendering (viewer).

## Screenshots

Map 1, seed 1, year 450. The files are in `docs/screenshots/overhaul/` as before/after pairs:
- R1–R2, summer, on v1.1: `1-overview`, `2-region` (zoomed to the central river valley) and `3-coast` (zoomed to the coast, river mouth and Chedesworth).
- R3, on v1.2: `4-fields` (zoomed to Chedesworth and Harnfield in summer) and `5-winter` (the central valley in winter).
- R4, on v2 G2: `6-roads` (the same view as `4-fields`, roads and routes before and after).
- R5, on v2 G3: `7-town` (Langden, a walled city, at full zoom; before is the per-cell view).
- R7: `8-lettering` (the whole map and panels). R6: `9-port` (Wenenford: quays on the smoothed shore, smoothed footpaths).
- R8: `10-hover-town` (the card over Langden, the filtered chronicle) and `10-hover-land` (the card over pasture by a road, with Dareswood selected).

## Possible next steps

The overhaul's planned steps are done. Ideas from the original proposal not yet built, none of them needed:

1. Style presets (`?style=atlas`, `?style=topo`) that swap the palette and symbols.
2. Fading between years at slow speeds, so forest, fields and footprints change smoothly.
3. The same smooth base map in the CLI's headless PNG renderer (`render.rs`), so milestone screenshots match the browser.
4. Rivers redrawn over roads inside towns (the D45 difference), if it is wanted.

## Release view tidy (implementer, v5 release)
Made by the implementer in the release pass, after R8 had been merged and handed over (D68). It touches layers the graphics session wrote, so it is recorded here.
- **Sea lanes bundled.**
  - `view.rs` keeps a `sea_lane` table, computed once per terrain: every sea cell within 4 km of land maps to the nearest sea cell next to the coast.
  - `routes.rs` sums segment flows on the mapped cells, so parallel coastal paths draw as one lane whose width is the total flow. `goods.rs` and `outside.rs` map their paths the same way, so goods and imports run along that lane as thin coloured lines.
  - Open-sea legs out to the map-edge ports keep their own cells.
- **Settlement names take priority.**
  - Each feature-name class (seas and bays, forests, ranges, rivers) first reserves the space the settlement names will take, by a dry run of the settlement placement. This happens only when settlement names are on.
  - A feature name that collides then tries four nearby positions, then hides.
- **Left for the graphics session.**
  - Feature names still do not avoid each other across classes. "Dew Forest", moved off "Everelford", now crosses "River Wilking" at year 450 on map 1. A shared placer across the feature classes would fix it.
  - Where routes leave the coastal lane for a port, a few separate strands remain.

## v7 release view (implementer)
Added in the v7 release pass, in new files where possible:
- **`layers/wool.rs`:**
  - pasture and sheep now come from the model's grazed land (`Snapshot.pasture`), and the relief shader's pasture channel reads the same mask;
  - wool routes to the ports draw in fleece cream.
- **`layers/prices.rs`:** a disc per place, coloured from cheap (blue) through the world price (pale) to dear (red).
- **Inspector:** grain price, market people, both purses, market store and pasture.

**For the graphics session, if wanted:**
1. **A smooth price surface** (an interpolated wash between places) instead of discs. Prices move together, so it would mostly show the level by year and the dearer inland edges.
2. **Purses as glyphs:** coin stacks at market places sized by the market purse, to show where coin pools (the ports and the wool towns).
3. **Pasture texture:** at the overview zoom pasture reads only as pale patches. Sheep are drawn from zoom 10. A downland texture (short grass, folds, dew ponds) would make the turn to grass visible from afar.
4. **Chronicle markers** on the map for "put down to grass" and the murrain, as for plague and silting.
