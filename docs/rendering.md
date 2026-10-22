# Rendering overhaul

Work on the viewer's look, done on the branch `claude/elegant-ride-g0sr1h` while engine work continues on `main`. Everything here is a view layer under the rule in the foundation (*if it can be recomputed from the state at any year, it is a view layer*): no change to `hinterlands-core`, `hinterlands-mapgen` or the CLI, and the golden hash is unchanged.

The entries below are numbered R1, R2… so they do not take decision numbers on `main`. R1 and R2 were merged into `main` as D36 and D37, R3 as D42, R4 as D43, R5 as D45, R6 as D46 and R7 as D47; later steps get the next free D numbers when they merge.

## Handover: merging into `main`

This section describes what is waiting to be merged; it is rewritten each time.

- **R1–R8 are merged** (D36, D37, D42, D43, D45, D46, D47, D68).
- **Building, not yet for handover:** the real-scale overhaul below. Stage 1 is R9, stage 2 is R10 and stage 3 is R11. By the owner's decision the stages are handed to the implementer as one batch, only after the current development stops; nothing has been sent.
- **Next step:** stage 4. The owner asked (22 October) that the remaining stages go ahead without waiting for approval, with towns the focus, and put optimisation off to a later stage so that the results are right first.

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

## R9 — Real scale, stage 1: the scale framework, and roads and rivers at true width

The first stage of the real-scale overhaul (below). The map now zooms in to about a metre per screen point, and roads and rivers are drawn into the terrain at their real width.

- **Zoom.** The deepest zoom is 1 m per screen point (about 0.5 m per physical pixel on a high-density screen): `cell_km × 1000` points per cell, 1,000 for the 1 km cells of maps 1–3. Before, it was 64 points per cell (15.6 m). The rules are in `scale.rs`; nothing assumes the cell size.
- **Scale bar.** In the map's lower-left corner: a round length of about 120 points, and the metres per point.
- **Symbols past the old deepest zoom.**
  - Icons, smoke, the selection ring, squares and bridges stop growing at 64 points per cell.
  - The map symbols of roads, rivers, trade flows (goods, wool, imports), footpaths, the trade dots and the bridge and ford glyphs fade out between 64 and 256 points per cell (15.6 to 3.9 m per point). There the true-width landscape has taken over.
  - **"Map symbols at full zoom"**, a new layer toggle (off by default), keeps them at full strength at every zoom. The routes layer is renamed "Roads and trade routes (map symbols)".
- **True width: the technique.**
  - **Lines.** The centrelines are the same runs the symbols use: the river paths and the year's road network (roads and land tracks).
  - **Smoothing.** Four rounds of corner cutting give segments of about 1/16 of a cell, so bends turn a few degrees at a time. They are uploaded as a data texture.
  - **Index.** Each square of a 250 m index grid lists the segments that can reach into it.
  - **Per pixel.** The shader takes the distance to the nearest road and the nearest river. Coverage is box-filtered, so a line thinner than a pixel fades instead of vanishing.
  - **When.** The pass runs once a pixel is under 15 m, which is past the old deepest zoom, and stops looking at a pixel more than 40 m from any line.
- **Widths, from the model.**
  - **Rivers:** 0.6 √A m, with A the drainage area in km². That gives 4.6 m where a stream starts (60 km²), 8.5 m where rivers become navigable (200 km²) and 55 m on the largest (8,460 km² on map 1).
  - **Estuaries** open to four times the width over the last 1.5 km. Spring widens every river by half, as the symbols do.
  - **Roads:** 3 m when quiet up to 8 m at full traffic, where "busy" is the routes layer's rule (full at a tenth of the largest flow). Land trade off the roads runs on 2.2 m tracks.
  - **Road condition** (`road_q`, D54) is not in the snapshot, so widths follow traffic alone.
- **Rivers, drawn.**
  - **Water:** shaded from the margins to the deeper middle, with streaks drawn out along the current. Ice forms at the edges in winter.
  - **Banks:** a muddy edge, and reeds in patches (fewer in woods, straw-coloured in autumn and winter). In a town with the buildings layer on, the banks are stone quays.
  - **Meanders:** the swing is 2 + 0.8 × the half-width in metres. The wavelengths are fixed, 90 m for brooks blending to 420 m for large rivers; a wavelength that followed the width shifted the phase wherever the width changed, and made the river shiver.
  - **Joins:** the swing dies away at the ends of each line, so rivers meet each other and the sea cleanly. A tributary keeps its own width to the join.
- **Roads, drawn.**
  - **Surface:** a worn surface the colour of the soil, darker on better land and dustier on busy roads. Wheel ruts run 1.4 m apart, one pair to a lane, and quiet lanes have grass down the middle.
  - **Edges:** grass verges, worn bare next to the road. Where a quiet lane cuts into a slope, the bank facing the light is lit and the other shaded.
  - **Highways** (busy above half, in open country): a ditch each side and a hedge beyond it, broken by gates, casting its shadow away from the light.
  - **Weather:** on low flat ground the road spreads by up to half again, where carts go round the mud. Puddles stand in spring and winter, and frost lies in winter.
  - **Towns:** paved through built-up areas.
  - **Bridges:** over water the road is a bridge, stone on busy roads and timber otherwise, with parapets.
- **Natural, not repetitive.**
  - **Wander.** Roads and rivers wander about 20 m either side of their course over most of a kilometre, and a few metres over a furlong, so nothing runs dead straight along the grid. They wander together, so they stay apart.
  - **Roads beside rivers.** A road that runs along a river, through the same cells, is moved out to the bank. It goes to the side it is already on, or the higher side if it lies on the river's line. Crossings stay where they are.
  - **Patterns** come from the integer hash of map position, as before, so they stay put under pan and zoom.
- **Toggles.**
  - "Roads at true width (close in)" is a new layer, on by default.
  - The rivers in the shader follow the Rivers toggle.
- **`?at=X,Y,Z`** opens the view centred on map point (X, Y) in cells at Z points per cell, for screenshots.
- **Left for later stages.**
  - **Stage 2:** footpaths at true width (they fade out close in for now), and fields and woods, which are still at the old scale: tree crowns about 250 m across, strips 166 m.
  - **Stage 3:** town footprints, still about 333 m street blocks. At full zoom they dwarf the true-width roads (`11-village-1m`).
  - **Stage 4:** bridges sized and placed at the model's bridges. Today any road over water is drawn as a bridge.
- **Checks.**
  - The golden hash is unchanged (`65aee272c30de1f9`), and `just test` passes.
  - The release wasm build gives the same hash at `?selftest=500` (30.5 ms/year).
  - No change to the core, mapgen or CLI.
- **Frame time.** The check ran in headless Chromium with SwiftShader, a software renderer far slower than any graphics card, so only the ratios mean anything. Median ms per frame, 1,400 × 900 window, map 1, year 450, around Asherfield:

  | View | Before | R9 |
  |---|---|---|
  | 64 points per cell (deepest before) | 550 | 633 |
  | 256 points per cell (4 m) | — | 683 |
  | 1,000 points per cell (1 m) | — | 783 |
  | 1,000, true-width roads and rivers off | — | 667 |

  - **At 64 points per cell** the true-width pass is skipped, yet frames are about 15% slower in SwiftShader. That is probably the larger shader; a graphics card skips a branch that is uniform across the frame.
  - **At full zoom** the pass adds about 17%.
  - **Still owed:** a check on real weak hardware.

Kind: rendering.

## R10 — Real scale, stage 2: fields, woods and pasture

The countryside drawn as it would look from the air at a metre a pixel. Each thing fades in once it is a few pixels across. The detail is invented, but where it lies comes from the cells: which land is farmed, wooded or grazed, and since when. The new shader code is in `country.frag`, spliced into `relief.frag`.

- **New per-cell data.** A ground texture carries:
  - wetness: a river cell, or one beside a river, on flat ground;
  - the year each cell was first farmed in this session;
  - the original forest cover.
- **Parcels.**
  - **Tessellation.** The farmland is cut into parcels of about 170 m, a Voronoi tessellation of scattered sites. Their edges bend about 30 m over a quarter of a kilometre and a few metres over a strip's width. Over most of a kilometre the parcels stretch and squeeze, so no lattice shows (see the check below).
  - **Which are farmed.** A parcel is farmed when the far-out field outline covers its site. Close in, the outline follows parcel edges, and it matches the far view to within a parcel.
  - **Uses.** About 60% of the farmed parcels are arable, as planned. The rest are meadow (most likely on wet land), common grazing, closes, and scrub.
  - **Assarts.** Where land was cleared late from woodland (original cover over about 0.45, first farmed after about 1220), arable parcels become small closes rather than open field.
- **Furlongs.**
  - **Strips:** 10–20 m wide, with irregular widths and a tone of their own. Each strip is a ridge, lit on the side facing the sun, higher on heavy land (good yield, flat), with the reversed S of the plough team. Rows run along the strip.
  - **Angle:** each furlong lies at its own angle around R3's rule (away from the nearest road or river). On wet or heavy sloping land the strips run downhill, to drain.
  - **Edges:** headlands of grass where the boundary crosses the strip ends, and narrow balks along their sides.
  - **Rotation:** furlongs are grouped into three great fields by a coarser tessellation of about 650 m. One field lies fallow each year and the others carry winter and spring corn, in R3's colours by season. In summer about a third of the winter-corn strips are already reaped.
- **Other parcels.**
  - **Meadows** are lush. In summer half of them are mown, with the hay in windrows about 6 m apart.
  - **Commons and scrub** are grazed turf.
  - **Closes** are each cut again into fields of about 70 m, under grass or ploughed in broad ridges ("lands").
  - **Old fields:** where fields went out of cultivation, ridge and furrow shows under the grass, fading over R3's 200 years.
- **Hedges.**
  - **Where:** round closes, round meadows, round the field block once it has been farmed fifty years (R3's rule), and between holdings.
  - **Age:** a hedge thickens as it ages, from about 1.6 m to 3.6 m wide over 50–400 years, and holds more standard trees (Hooper's rule). The trees are oak or birch, 3–7.5 m across, one by chance in each stretch of about 40 m.
  - **Detail:** gates now and then. The hedge and its trees cast shadows away from the light. Far out, a hedge stays a fine line so the field pattern reads.
- **Woods.**
  - **Outline.** The wood's outline is the old cell-scale one, refined by finer noise at 90 m and 25 m as you zoom in, so its shape holds across zooms.
  - **Far out** a wood is a tint, as before. From about 25 m per pixel it is a texture of stands and groups, and from about 7 m per pixel single crowns appear.
  - **Crowns:** 3.5–8 m across (up to 10 m on lone trees), domed and lit, with shadows. The woodland floor shows between crowns inside the wood.
  - **Species** follow the ground: alder and willow by wet streams, beech on dry upland, birch on poor land, oak elsewhere. Trees of a kind grow in stands of about 150 m. Each species has its own colour by season (beech orange in autumn, and so on), and deciduous trees are bare in winter.
  - **Lattice.** Trees come from three jittered grids at different angles and sizes, each holding a third of them. With one grid, the 11 m lattice showed in the repetition check (spectral peak 93; 8 with three).
  - **Old wood** (dense cover) has larger crowns. Lone trees past the wood's edge are fewer and broader.
  - **Coppice** (cover cut below its original, the rule of the coppice glyphs): hazel underwood in coupes of about 80 m, each at its own stage of regrowth (most grown back, a few newly cut), under scattered oak standards.
- **Downland.** On the model's grazed land:
  - grazed turf mottled at three scales, rank in patches;
  - gorse and thorn;
  - dew ponds on the upland, about one to every 700 m square;
  - sheep folds, hurdle pens on trampled ground.

  Far out, tussock marks, eight to a cell, draw rough grazing the old way, so the turn to grass shows across the map (the implementer's request in the v7 notes).
- **Open land.** Open land that is neither farmed, grazed nor wooded gets the same turf texture, with thorn thin on poor land.
- **Footpaths at true width** (left over from R9):
  - the footpaths layer's walk, every five years, drawn as 1.2 m trodden lines that wander a little, with plank bridges over streams;
  - kept off the rivers like the roads;
  - they follow the Footpaths toggle, and the symbol still fades out close in.
- **Retired.** These are replaced at every zoom:
  - the old six strips per cell;
  - the cell-scale hedges and ghost-strip lines;
  - the 250 m tree crowns.

  Far out, the field and forest tints are unchanged.
- **Toggles.** Each part follows its layer as in R3:
  - furlongs, meadows and closes follow Fields;
  - strips and ridges follow Strips;
  - commons and downland follow Pasture;
  - hedges and old fields follow Hedgerows and old fields.
- **Repetition check.**
  - **The tool.** `tools/rhythm.py` measures the sharpest peak of a screenshot's power spectrum. It compares each peak with both the spectrum's level at that radius and its neighbours along the same ray, so straight lines and edges, which spread along a ray, do not count. A score above 30 is a rhythm. The autocorrelation peak is reported for diagnosis only.
  - **Results.** Before (R9) and after (R10), at the same views:

    | View | R9 | R10 |
    |---|---|---|
    | 64 points per cell | 18.5 | 14.2 |
    | 4 m, Asherfield | 22.6 | 44.6 |
    | 4 m, town blanked out | 6.7 | 7.2 |
    | 1 m, highway in fields | 54.4 | 37.1 |
    | 1 m, wood | 7.0 | 8.0 |
    | 64, downland | 8.8 | 10.0 |

  - **4 m view.** The rhythm there is Asherfield's street grid of 333 m blocks (period 41 px, about 170 m). It is unchanged from R5 and is replaced in stage 3. With the town blanked out, R10 scores 7.2. R9's score at that view also held its old six strips per cell.
  - **1 m view.** The score comes from the highway's parallel bands (ditch, verge, hedge). Without the road the view scores 10.9.
- **Frame time.** Not optimised in this stage, by the owner's decision. Median ms per frame in headless Chromium with SwiftShader (a software renderer, so only the ratios mean anything), measured one after another in the same session. R9 measured slower here than in R9's own check because the machine was loaded differently.

  | View | R9 | R10 |
  |---|---|---|
  | 64 points per cell | 733 | 1,200 |
  | 1 m, highway in fields | 733 | 3,000 |
  | 1 m, wood | — | 2,533 |
  | 4 m, Asherfield | — | 3,250 |

  At full zoom stage 2 costs about four times stage 1. Most of it is the parcel tessellation (with both parcels' data for hedges), and the three tree grids, each searched twice for shadows. These are the first targets when optimisation comes: the planned cache of the rendered map, levels of detail, and a parcel texture built on the CPU.
- **Checks.** The golden hash is unchanged (`65aee272c30de1f9`), `just test` passes, and `?selftest=500` matches. No change to the core, mapgen or CLI.

Kind: rendering.

## R11 — Real scale, stage 3: villages and towns

Each place is laid out the way it grew, from its roads, by a planner on the CPU (`town.rs`, rebuilt each year, every 5 years while the run plays). The relief shader draws the result (`town.frag`). The owner's notes for this stage:
- the roads to other places must pass through the towns;
- buildings gather round the roads;
- a town's own streets must look like roads;
- the whole must sit in the land, not look like a sticker on it;
- fields must end at a real town boundary, not be painted over.

- **Main streets are the roads.**
  - The roads through a place (the model's roads, smoothed and kept off the rivers as in R9) are its main streets. The centre is the nearest dry point on them to the anchor. A bridge town's centre is on the nearer bank.
  - Towns ignore land-trade tracks where they have a road. Villages, hamlets and roadless towns use their tracks.
  - A place on no road or track gets a high street of its own, laid from the nearest dry ground the way that keeps longest on it.
- **Inner streets.** They are drawn exactly like the roads: the same true-width shader, worn surface, ruts and verges, and paved where the place is a town.
  - **Side streets** branch off the main streets, roughly every 80–130 m on alternate sides, bending a little as they go. Larger towns grow them in generations: lanes off lanes, two generations from 1,500 people and three from 5,000. A town on only one or two roads grows two more, further out.
  - **Back lanes** run behind the main streets' plots in towns of 800 or more.
  - **Spacing.** Every lane keeps a plot's depth from every street but the one it leaves. So lanes join into blocks behind the street fronts, never across them, and a lane ends where it meets another's frontage.
  - **Ends.** A place's own lanes are cut back to the last point inside its boundary, so none runs on into the fields.
- **Plots, one per household** (people ÷ 4.5).
  - **Order:** laid along every street from the centre outward, main streets first.
  - **Hamlets:** about 32 m frontages.
  - **Villages:** tofts and crofts about 20 m wide and 70 m deep.
  - **Towns:** burgage plots narrowing from 9 m to 5.5 m in the largest cities, 70–45 m deep, shorter near the centre where land was dear. Plots widen towards the edge of a place.
  - **Inside a plot:** a yard about the first 12 m behind the house, a kitchen garden of beds, then grass, an orchard of fruit trees or the croft's own crop at the back. Each plot is fenced, with wattle and walls in towns and hedges in villages.
  - **Water:** no plot or house stands in water or on a river's banks.
- **Houses.**
  - **Towns:** the whole frontage, 4–6 m deep and 8–11 m high.
  - **Villages:** a longhouse 10–15 m long, along the street or end on (three in ten), set back a little.
  - **Barns and back ranges:** about half the plots have a barn behind the house, or in a town a back range down one side.
  - **Roofs:** thatch golden to grey, red-brown tile courses (more of them the larger the town), and stone slates on the upland. Each roof is lit on its slopes, with a ridge and (most houses) a chimney, eaves, and its shadow cast away from the light. Roofs whiten in winter.
- **Market places and greens.**
  - Towns of 500 or more have a spindle-shaped market place on the main street at the centre, cobbled, with a cross and stalls. From 1,500 people a market hall stands in it.
  - Villages where three or more roads meet have a green, often with a pond.
- **Churches.** One to about a thousand people, in a churchyard with gravestones and a wall: the first by the centre, the rest spread out. Each faces east, with a tower at the west end and a chancel narrower than the nave, roofed in lead in towns.
  - **Cathedral:** the greatest city, or one over 8,000, has a cathedral (110 m long, with transepts and a cloister) in a close.
  - **Friaries:** cities have one per 4,000 people, a precinct with church and cloister near the edge.
- **Walls.**
  - **Line:** a city's wall goes round about half its first layout of plots, with room for a lane inside. The plots are then laid again clear of it, so the rest form suburbs along the roads beyond.
  - **Wall:** a crenellated wall walk, round towers about every 50–65 m, and a wet ditch outside.
  - **Openings:** a gatehouse where each road passes, and a postern (a gap without a gatehouse) where a lane does.
- **The town's edge, and the fields** (revised after the owner's review; see below).
  - **Edge:** each place's edge comes from its own plots. The squares of the 62.5 m town index that hold any of its ground (plots, courts, yards, greens, the market) count as town land. The value falls away to nothing about three squares out. The edge is the 0.5 line of that field, smoothed, which lies some 80 m beyond the last plots and wanders at 140 m and 35 m.
  - **Fields** (R10's parcels) stop at the edge, decided per pixel, so no field runs under a house. Inside it the land is the place's own closes, about 60 m across: paddocks of grass (most), orchards in rough rows, kitchen gardens, and, only on the fringe away from the houses, closes under crops. About a third of their sides are open, and the rest have younger trimmed hedges.
  - **Edge hedge:** the edge itself is hedged, with a gate now and then.
  - **Woods** stop at the edge too, with a broken line.
  - **Lanes** are cut back to the last point inside each place's radial boundary (64 directions round its plots, kept for this and for the area figures).
  - **The old R5 drawing** (footprint wash, 333 m street blocks, walls on the cell outline) is retired. Built cells count as farmland in the far-out field tint, and their woods drop to a quarter.
- **Far out,** a place is drawn as the share of the ground its plots cover, in the colour of roofs, fading out as the houses come in (from 15 m per pixel).
- **Ruins** (deserted places): far out, R5's grey patch; close in, the earthworks of the crofts under grass, with their banks and house platforms.
- **Quays** on the sea follow the new plots. Riverbanks are stone quays in towns, as in R9. In a town the river is embanked and does not meander.
- **Symbols.** The settlement marker, the church, square and warehouse glyphs and the price discs fade with the other map symbols past 64 points per cell. The mill stays until stage 4.
- **Revision after the owner's review (22 October).** The owner's points:
  - fields showed under the buildings, through gaps in the old radial boundary;
  - Dewermouth looked like a wheel with long spokes;
  - large cities could be denser, with other kinds of housing.

  The changes:
  - **The edge** is now the town-land field described above, replacing the radial boundary, and the green wash over the fields is gone.
  - **Tenements and courts.** In the core of cities over about 4,000 people (full at 15,000), a plot holds up to four households. The front house is a storey taller and deeper, let as tenements. From three households the yard becomes a court: trodden ground, a row of small cottages down one side, an alley beside them. This is how large medieval towns grew denser, by building over their plots rather than spreading.
  - **Compact cities.** Cities of 5,000 or more grow four generations of lanes, about 70 m apart. The room the tenements save is subtracted from the plots the planner looks for.
  - **Dewermouth** now reaches about 1.3 km from its centre, against 2.0 km before. It still houses only about two-thirds of its households, and keeps two longer arms along its north and north-east roads.
- **Sizes against the curve.** Settled area inside each boundary against Cesaretti et al.'s curve (plus crofts for villages), map 1 seed 1, places over 1,000 people:

  | Year | Place | People | Houses / households | Area | Expected |
  |---|---|---|---|---|---|
  | 200 | Asherfield (walled) | 4,507 | 1,002 / 1,002 | 56 ha | 56 ha |
  | 200 | (id 24, walled) | 2,271 | 505 / 505 | 34 ha | 34 ha |
  | 200 | (id 30) | 1,732 | 385 / 385 | 27 ha | 28 ha |
  | 450 | Liningbourne (walled) | 5,885 | 1,308 / 1,308 | 66 ha | 68 ha |
  | 450 | Verefield (walled) | 2,479 | 551 / 551 | 41 ha | 36 ha |
  | 450 | Ordcote | 1,884 | 419 / 419 | 25 ha | 30 ha |
  | 450 | Dewermouth (walled port) | 15,041 | 2,243 / 3,342 | 92 ha | 134 ha |
  | 450 | Aldford (port) | 1,293 | 201 / 287 | 20 ha | 29 ha |
  | 450 | Mereelwick (river, no road) | 1,035 | 132 / 230 | 14 ha | 42 ha |

  Most places house every household, within about a sixth of the expected area.

  (Houses: households housed since the revision, so tenements count each household.)

  **Known shortfall.** Places cramped between river, sea and their few streets fall short of houses. Dewermouth houses 67%, Aldford 70%, Mereelwick 58%. The planner's summary is logged at info level for checking.
- **Repetition check** (`tools/rhythm.py`, R10 against R11, same views):

  | View | R10 | R11 |
  |---|---|---|
  | 4 m, the walled city of 1300 | 42.2 | 15.5 |
  | 1 m, the same city | 17.2 | 15.1 |
  | 1 m, Asherfield's highway, 1550 | 15.1 | 9.2 |
  | 4 m, Dewermouth | 13.2 | 12.9 |
  | 64 points per cell | 13.9 | 14.6 |

  The street grid's rhythm (R10's one failing view) is gone.
- **Frame time.** Not optimised in this stage (the owner's decision). At full zoom over Dewermouth a frame takes up to about 50 s in SwiftShader, against about 3 s for R10's fields. The cost is the per-pixel scan of up to 160 town shapes and their shadows. This, with the parcels and trees, is the work for the optimisation stage.
- **Checks.**
  - The golden hash is unchanged (`65aee272c30de1f9`), and `just test` passes.
  - The release wasm build gives the same hash at `?selftest=500` (30.4 ms/year).
  - No change to the core, mapgen or CLI.

Kind: rendering.

## Screenshots

Map 1, seed 1, year 450. The files are in `docs/screenshots/overhaul/` as before/after pairs:
- R1–R2, summer, on v1.1: `1-overview`, `2-region` (zoomed to the central river valley) and `3-coast` (zoomed to the coast, river mouth and Chedesworth).
- R3, on v1.2: `4-fields` (zoomed to Chedesworth and Harnfield in summer) and `5-winter` (the central valley in winter).
- R4, on v2 G2: `6-roads` (the same view as `4-fields`, roads and routes before and after).
- R5, on v2 G3: `7-town` (Langden, a walled city, at full zoom; before is the per-cell view).
- R7: `8-lettering` (the whole map and panels). R6: `9-port` (Wenenford: quays on the smoothed shore, smoothed footpaths).
- R8: `10-hover-town` (the card over Langden, the filtered chronicle) and `10-hover-land` (the card over pasture by a road, with Dareswood selected).
- R9, on v7 (year 450, summer unless named), around Asherfield, a village of 475 on a busy road:
  - `11-village-before` is the deepest zoom before R9 (64 points per cell); `11-village-after` is the same view after;
  - `11-village-4m` is at 4 m per point and `11-village-1m` at 1 m;
  - `11-highway-1m` is the highway leaving the village, `11-highway-1m-winter` the same in winter, and `11-highway-1m-symbols` the same with "Map symbols at full zoom" on;
  - `11-brook-1m` is the brook into Chaleford;
  - `11-stream-road-2m` is Caley at 2.5 m: a lane kept to the bank of its stream, and a bridge over the larger river;
  - `11-estuary-4m` is the mouth at Dewermouth.
- R10, the same village (year 450, summer unless named):
  - **Pairs** (R9, then R10): `12-village-64-before`/`-after` (64 points per cell), `12-village-4m-before`/`-after`, `12-fields-1m-before`/`-after` (furlongs, closes and a copse by the highway), `12-wood-1m-before`/`-after`, and `12-downland-64-before`/`-after` (the tussock marks, round Falecote on the upland).
  - **Close views:** `12-closes-1m` (small hedged closes and ridge and furrow under grass), `12-wood-edge-2m`, `12-coppice-1m` and `12-downland-1m` (dew pond, folds, gorse).
  - **Seasons:** `12-autumn-2m` and `12-winter-2m`.
  - **Year 200 (1300), at the height of the open fields:** `12-open-fields-1300-4m` and `12-open-fields-1300-1m`.
- R11 (R10, then R11, unless named):
  - **Asherfield, 1550:** `13-village-64-before`/`-after`, `13-village-1m-before`/`-after` (a street village along the highway).
  - **Asherfield, 1300,** then a walled city of 4,500: `13-city-1300-4m-before`/`-after`, `13-city-1300-1m-before`/`-after`, and `13-city-1300-winter`.
  - **Dewermouth** (the port city of 15,000 at the river mouth): `13-port-4m-before`/`-after` and `13-port-1m`.
  - **Others:** `13-upland-town-2m` (Liningbourne, stone-slate roofs) and `13-river-town-2m` (Mereelwick).

## Next: the real-scale overhaul (in progress: stages 1–3 are R9–R11)

Recorded on 22 October 2026 from the owner's conversation with the graphics session.

### Status and rules
- **Status:** stages 1–3 built (R9–R11); stage 4 next, without waiting for approval (the owner, 22 October). It replaces the earlier waiting list: the town sizes, the field proportions and the traffic pacing are all folded in here.
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

### Queued after stage 4: towns that are not repetitive within themselves
Asked for by the owner on 22 October, to follow once stage 4 is done.
- **Kinds of building.** Vary the shapes and sizes of buildings within a town, not just their roofs and tones:
  - L-shaped and courtyard houses, and hall houses set back behind a yard;
  - long ranges, inns with yards, merchants' houses with warehouses behind;
  - cottages and rows;
  - wider frontages where plots have been thrown together.
- **Districts.** Let a town have quarters of different character:
  - the market and its streets: tall, narrow and fully built frontages;
  - the waterfront: quays, warehouses and cranes;
  - craft streets: workshops, yards, tanneries by the water downstream;
  - the precincts of churches and friaries;
  - poorer edges: cottages and gardens;
  - suburbs along the roads beyond the gates.

  Which district is where follows the town's roads, water and centre, so each town differs because its site does.
- **Dewermouth's remaining arms and shortfall** are the first test of it.

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
