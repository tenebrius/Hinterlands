# Rendering overhaul

Work on the viewer's look, done on the branch `claude/elegant-ride-g0sr1h` while engine work continues on `main`. Everything here is a view layer under the rule in the foundation (*if it can be recomputed from the state at any year, it is a view layer*): no change to `hinterlands-core`, `hinterlands-mapgen` or the CLI, and the golden hash is unchanged.

The entries below are numbered R1, R2… so they do not take decision numbers on `main`. R1 and R2 were merged into `main` as D36 and D37, R3 as D42, R4 as D43, R5 as D45, R6 as D46 and R7 as D47; later steps get the next free D numbers when they merge.

## Handover: merging into `main`

This section describes what is waiting to be merged; it is rewritten each time.

- **Nothing is waiting.** R1–R8 are merged (D36, D37, D42, D43, D45, D46, D47, D68).
- **Next step:** the real-scale overhaul below is planned but not started. By the owner's decision it is built and handed over only after the current development stops.

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

## Next: the real-scale overhaul (planned, not started)

Recorded on 22 October 2026 from the owner's conversation with the graphics session.

### Status and rules
- **Status:** planned, not started. It replaces the earlier waiting list: the town sizes, the field proportions and the traffic pacing are all folded in here.
- **When:** the owner asks that it be built and handed to the implementer only after the current development stops, so as not to disturb the designer and implementer.
- **Approval:** frontend only, so the owner's approval stands in for the designer's.
- **Handover:** it goes to the implementer as one batch, built in stages (below) so that each stage can be judged on screen.
- **Unchanged:** the core, mapgen and CLI, and the golden hash.

### Why
The owner's aims:
- roads should not be drawn as lines, but textured into the terrain at their real width;
- a map-symbol overlay of roads can be switched on for a clear view;
- the map should zoom in far enough to see roads at that width;
- everything else should look right at that zoom too.

Today the deepest zoom is 64 px per 1 km cell, about 16 m per pixel, and the procedural detail is drawn at the wrong scale. Tree crowns come out at 250 m, field strips at 166 m, town blocks at 333 m and brooks at 60 m. Towns cover 5–10 times their real area.

### Figures that set the numbers
These were measured, or looked up, during the planning:
- **The model's fields are realistic.** Maps 1–3 at year 450 feed 62–65 people per km² of farmed land, about 1.5 ha per person, towns included. That matches a medieval township if about 60% of each farmed km² is open-field arable in a three-field rotation and the rest is meadow, common pasture, closes and waste. The sown land then yields about 0.44 t/ha net of seed, roughly 9 bushels an acre gross, in line with English demesne yields around 1300 as Campbell's yield work describes them. So the picture has to change, not the model.
- **One food unit is one person's grain for a year.** That is about 250 kg of threshed grain: about 2,000 kcal a day, some 80% of it from grain (Dyer), at 2,500–2,800 kcal per kg as bread, pottage and ale. The trade-dot layer already uses 0.25 t per unit. The viewer should show tonnes beside units.
- **Town density.** In medieval European cities, settled area (suburbs and open ground within the walls included) runs to roughly area_ha ≈ 0.12 · N^0.73 for English towns. That is about 55 people per ha at 1,000 people and about 100 per ha at 10,000 (Cesaretti et al. 2016, PLOS ONE; derived from their fitted curve). Market places, churches, friaries and gardens are inside that area. Today the footprint is drawn at 1,500 people per km² (`density_per_cell`, rendering only) with at least one whole cell per place: about 11 per ha for towns, and 100 ha for a village of 250. Real villages with their tofts and crofts covered roughly 15–25 ha, an estimate from plot sizes.
- **Real widths:**
  - lanes 2–4 m, highways 5–10 m with verges and ditches;
  - brooks 1–5 m, navigable rivers tens of metres;
  - tree crowns 6–14 m;
  - ridge-and-furrow strips 10–20 m wide in furlongs of about 200 m;
  - houses about 10 × 5 m, burgage frontages 5–6 m, streets 5–10 m.
- **Traffic volume.** At year 350 on map 1 there are about 3,800 flows a year on paths averaging 45 cells. At about 1 t a cart, that is roughly 25 cart departures a day across the map, and about 60 carts on the road at any moment.

### Principles
1. **Everything in metres,** fading in at the zoom where it is a few pixels across. The deepest zoom goes from 64 to about 500–1,000 px per km (1–2 m per pixel).
2. **Two styles for roads and rivers:**
   - map symbols: today's exaggerated lines, kept as an overlay toggle;
   - the real landscape (the default): true width and texture, blending in as you zoom.
3. **Invented detail, never invented quantities.** Below 1 km everything is procedural, but it is constrained by the model:
   - one house per household (people ÷ 4.5);
   - fields covering exactly the farmed land, about 60% of it strips;
   - roads exactly where the model's roads run, at widths from their traffic and condition;
   - towns sized from population by the curve above.
4. **No change of graphics engine.** It stays on eframe with glow (WebGL2), with the dense detail per pixel in the relief shader. egui shapes are kept for labels, symbols and moving things.
   - **Possible later:** wgpu only if compute shaders or mass instancing are ever needed. It would mean porting GLSL to WGSL, a few days' work.
   - **Ruled out:** a game engine such as Bevy.
5. **Cache the rendered map** off-screen while the camera and year are unchanged; only moving things are drawn every frame.

### Components
- **Scale framework:** the zoom range, scale bands, and every size in metres via `cell_km`.
- **Roads and rivers at true width.**
  - **Technique:** the smoothed centrelines (`lines::chains`, Chaikin) are uploaded as a data texture with a per-cell segment index, and the shader takes the distance to the nearest centreline.
  - **Roads:**
    - a worn surface tinted by the soil;
    - ruts, worn verges, and ditches and hedges along highways;
    - sunken lanes on slopes, puddles in winter;
    - width classes from the model's road condition and traffic.
  - **Rivers:** true width with banks and reeds; estuaries widen.
  - **Overlay:** the routes layer becomes the map-symbol overlay.
- **Fields at real scale:**
  - ridge-and-furrow strips and furlongs, with headlands;
  - hedges with standard trees;
  - meadows along streams, commons and small closes;
  - about 60% strips per farmed cell.
- **Woods at real scale:** individual crowns with shadows close in, and a texture further out. Includes the downland pasture texture the implementer asked for (short grass, folds, dew ponds).
- **Villages and towns at real scale:**
  - built-up area from population;
  - houses on tofts and burgage plots, streets;
  - the market place where the main roads meet, sized to the town;
  - churchyards, one more per thousand or so people;
  - friary or abbey precincts and a cathedral close in cities;
  - walls round about half the settled area in walled towns, suburbs along the roads beyond the gates.
- **Buildings to size:** bridges, mills, quays, walls and churches drawn to real size close in, in place of icons.
- **Traffic and pacing** (it replaces the 1 s = 1 day dots):
  - **Paused, "a day on the roads":** vehicles to scale (ox-carts in convoys, packhorse trains, barges, cogs), coloured by cargo. The time rate follows the zoom, so a cart crosses the screen at 30–60 px a second (about 1–6 hours per second), and a label shows the rate. Vehicle numbers are true to the trade volume, with an optional labelled "traffic ×5". The season holds still while paused.
  - **Playing up to about 4 years/s:** flow dashes along the routes, their density from the yearly volume and their speed from the mode.
  - **Playing faster:** still lines whose widths ease between years.
  - **Moving between paces:** fades rather than jumps.
- **Readouts:** tonnes beside food units in the hover card and inspector.
- **From the implementer's notes, folded in where they fit:**
  - one shared label placer across the feature classes;
  - chronicle markers on the map (grass, murrain);
  - a smooth price surface, as an option to the price discs.

### Natural, not repetitive
Repetition shows when the eye finds the same pattern twice at the same scale. The rules:

1. **Vary detail by cause, not just by noise.** The snapshot already carries the causes:
   - **Woods:** the tree mix follows the ground: alder and willow by wet streams (flux), beech on dry upland, oak on heavier land. Crowns are larger in old wood and smaller in coppice.
   - **Fields:** strips run with the slope where the land needs draining, and their direction changes from one block of strips (furlong) to the next. Ridges are higher on heavy land.
   - **Fields by age, from `farm_first`:** land farmed early is open-field strips; land cleared late from woodland (assarts) is small irregular hedged closes.
   - **Hedges:** older hedges are thicker with more trees (Hooper's rule: about one shrub species per century).
   - **Villages:** the layout follows the site: a street village along a road, a green village at a crossroads, a scattered cluster on poor upland.
2. **Build things the way they grew, rather than stamping patterns.**
   - **Towns:** narrow plots run back from the street, laid out from the market place outward; they fill in more tightly near the centre, and houses face the street. Each town differs because its roads do.
   - **Roads:** they follow the terrain and widen where carts spread around mud.
3. **Vary at three scales at once:** each feature on its own (each tree's size, shade and shape); its group (a stand of trees, a furlong sown with one crop); and its district (soil colour, wetness, height).
4. **No grids and no straight lines.**
   - Points are scattered evenly but irregularly (blue noise or Poisson-disc), never on a hidden lattice.
   - Each furlong gets its own angle.
   - Hedges and plot edges get a slight wobble at low frequency; strips keep the reversed S.
   - The hash is an integer hash, so no regular pattern bleeds through from it.
5. **Small imperfections, kept stable over time.**
   - Each strip has its own tone; some are harvested while others still stand; grass is worn near gates and paths; roofs shade unevenly.
   - All of it is seeded from things that persist (cell, owner, `farm_first`), so a wood keeps its trees from year to year and detail changes only when the model says something changed.
6. **Check against the real thing.**
   - Side by side with real references at every zoom: early Ordnance Survey and enclosure maps, aerial photos of surviving ridge and furrow, Laxton's open fields.
   - An automated check: measure how strongly a screenshot repeats itself at regular intervals (the peaks of its spectrum, or its autocorrelation), and fail a stage that brings in a visible rhythm.

These rules add roughly 10–15% to the overhaul, and they apply in every stage.

### Stages
1. **Scale framework, plus roads and rivers at true width.** Judged on one village and its surroundings at full zoom.
2. **Fields and woods at real scale,** with the pasture texture.
3. **Villages and towns at real scale,** with the town-size fix.
4. **Buildings to size, and traffic with the new pacing,** plus the readouts and the folded-in notes.

**Each stage is checked by:**
- golden unchanged; `?selftest=500` matching;
- before/after screenshots;
- a frame-time check at full zoom on weak graphics hardware.

**Size:** about 3,000–5,000 lines, mostly shader.

**Risks:**
- per-pixel cost at full zoom on weak hardware, met by levels of detail and the cache;
- invented detail looking repetitive.

### Earlier ideas, not scheduled
1. Style presets (`?style=atlas`, `?style=topo`).
2. Fading between years at slow speeds.
3. The smooth base map in the CLI's PNG renderer (`render.rs`).
4. Rivers redrawn over roads inside towns (the D45 difference). The real-scale rivers make this moot.

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
