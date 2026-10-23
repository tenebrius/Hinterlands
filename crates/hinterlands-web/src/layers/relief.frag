// Smooth relief: the terrain cells are interpolated per pixel instead of drawn
// as squares. Inputs are the cell values only; everything here is decoration
// derived from them (coastline, shallows, hillshade, canopy) and never feeds back.

precision highp float;
precision highp int;
precision highp sampler2D;

// r: elevation (m), g: signed distance to the coast (cells, land positive),
// b: food yield, a: hillshade (1 on the flat).
uniform sampler2D u_land;
// This year, r: forest cover, g: cultivated, b: pasture, a: abandoned-field fade.
uniform sampler2D u_cover;
// This year, r: owner (hashed, -1 for none), g: strip direction + 2 x rotation,
// b: hedged (farmed 50 years), a: flood meadow.
uniform sampler2D u_plot;
// This year, r: built up, g: ruin, b: inside a city's walls, a: how fresh the ruin
// is, or on built cells, whether the settlement is a town or larger (quays).
uniform sampler2D u_town;
// Settlements and buildings layers on.
uniform vec2 u_town_show;
uniform float u_year;
// 0 spring .. 3 winter; u_season is -1 with the seasons layer off.
uniform int u_season;
uniform int u_field_season;
// Layers on: fields, strips, pasture, hedgerows and old fields.
uniform vec4 u_show;
uniform vec2 u_grid;
// Map coordinates (cells) of the top-left and bottom-right of the drawn rect.
uniform vec2 u_p0;
uniform vec2 u_p1;
// Physical screen pixels per cell.
uniform float u_px;
// Roads and rivers at true width (R9). Segments, two texels each:
// (x0, y0, x1, y1) in cells, then (half-width at each end in metres,
// 4 x kind + busy, metres along the line at the start). Kinds: 0 stream,
// 1 navigable river, 2 track, 3 road, 4 footpath, 5 town street. On rivers, busy is replaced by the
// kilometres to the end of the line (up to 0.999).
uniform sampler2D u_ways;
// Per square of the index grid (u_wsub squares to a cell along a side): first
// entry in u_widx and how many segments reach into the square.
uniform sampler2D u_wcell;
uniform float u_wsub;
// Segment numbers, four to a texel.
uniform sampler2D u_widx;
// True-width roads on, rivers on, spring widening of the rivers, footpaths on.
uniform vec4 u_way_show;
// Metres per cell.
uniform float u_cell_m;
// The look (R12): roads, rivers, trees and buildings, each against its true size.
uniform vec4 u_look;
// Ink outlines (R14), 0..1: dark edges round roofs, crowns, rivers, roads and
// walls, as on hand-drawn maps, about a pixel wide at any zoom.
uniform float u_ink;
const vec3 INK = vec3(0.17, 0.15, 0.13);
// The metres detail is drawn in: 1 for true metres, or a look's scale while
// drawing roads, trees or buildings, whose own detail (verges, crowns, ridges)
// is laid out in their own metres and grows with them. `mpx` divides by it.
float g_unit = 1.0;
// Metres from the point to the nearest building of a town (R13; 1e9 when far
// from any), set before the land is drawn so trees keep off the houses.
float g_bnear = 1e9;
// Metres from the point to the nearest road, track, street or path's edge
// (R23; 1e9 when far from any), set before the land is drawn so trees keep off
// the roads.
float g_road_sd = 1e9;
// The pixel's position in metres (R27).
vec2 g_pm = vec2(0.0);
// Whether the crowns being drawn include a place's groves (R27), whose density
// is then taken at each tree's own centre (`grove_at`) rather than carried
// from the pixel, so a tree is kept or not as a whole.
bool g_grove_on = false;
float grove_at(vec2 pm);
// Metres from a point to the nearest road's edge, and to the nearest building
// or open ground of a town (defined below).
float road_sd_at(vec2 pm);
float town_sd_at(vec2 pm);
// Whether a tree of radius r (metres) centred at c (metres) stands clear of
// the roads and the buildings, walls and open grounds of towns: decided at
// its centre, the same for every pixel of its crown, so a crown is whole or
// absent, never cut (R27). Far from any, by the distances at the pixel, it
// is clear without looking further.
bool tree_clear(vec2 c, float r) {
    // Always measured at the centre itself, never estimated from the pixel:
    // the pixel's own distances only see the shapes listed for its square,
    // so pixels in neighbouring squares disagreed and crowns broke into
    // crescents along the squares' edges.
    // Shortcut: the pixel's own distances are exact within the reach of its
    // square's lists (40 m for roads, 14 m times the building scale for
    // towns); a tree whose whole clearance lies within that and is clear of
    // the pixel's distance is clear at its centre too, by the triangle
    // inequality, so every pixel agrees.
    float off = length(c - g_pm) + 2.0;
    float need_r = r + 0.5 * u_look.z;
    bool road_ok = u_way_show.x < 0.5
        || (off + need_r < 40.0 && g_road_sd > off + need_r)
        || road_sd_at(c) > need_r;
    if (!road_ok) {
        return false;
    }
    float need_b = r + 1.0 * u_look.z;
    return u_town_show.x < 0.5
        || (off + need_b < 14.0 * u_look.w && g_bnear > off + need_b)
        || town_sd_at(c) > need_b;
}

in vec2 v_uv;
out vec4 o_color;

vec4 land_at(ivec2 c) {
    return texelFetch(u_land, clamp(c, ivec2(0), ivec2(u_grid) - 1), 0);
}

vec4 cover_at(ivec2 c) {
    return texelFetch(u_cover, clamp(c, ivec2(0), ivec2(u_grid) - 1), 0);
}

vec4 town_at(ivec2 c) {
    return texelFetch(u_town, clamp(c, ivec2(0), ivec2(u_grid) - 1), 0);
}

// Smoothed town field and its gradient (per cell), for distances to its outlines.
void town_smooth(vec2 p, out vec4 v, out vec4 gx, out vec4 gy) {
    vec2 q = p - 0.5;
    vec2 b = floor(q);
    vec2 f = q - b;
    vec2 s = f * f * (3.0 - 2.0 * f);
    vec2 ds = 6.0 * f * (1.0 - f);
    ivec2 ib = ivec2(b);
    vec4 v00 = town_at(ib);
    vec4 v10 = town_at(ib + ivec2(1, 0));
    vec4 v01 = town_at(ib + ivec2(0, 1));
    vec4 v11 = town_at(ib + ivec2(1, 1));
    v = mix(mix(v00, v10, s.x), mix(v01, v11, s.x), s.y);
    gx = ds.x * mix(v10 - v00, v11 - v01, s.y);
    gy = ds.y * mix(v01 - v00, v11 - v10, s.x);
}

// Signed distance in cells to the 0.5 line of a smoothed field (positive inside).
float field_dist(float v, float gx, float gy) {
    return (v - 0.5) / max(length(vec2(gx, gy)), 0.05);
}

vec4 plot_at(ivec2 c) {
    return texelFetch(u_plot, clamp(c, ivec2(0), ivec2(u_grid) - 1), 0);
}

// Catmull-Rom weights.
vec4 cr(float t) {
    float t2 = t * t;
    float t3 = t2 * t;
    return vec4(
        -0.5 * t3 + t2 - 0.5 * t,
        1.5 * t3 - 2.5 * t2 + 1.0,
        -1.5 * t3 + 2.0 * t2 + 0.5 * t,
        0.5 * t3 - 0.5 * t2);
}

vec4 land_row(ivec2 b, int dy, vec4 wx) {
    return wx.x * land_at(b + ivec2(-1, dy)) + wx.y * land_at(b + ivec2(0, dy))
        + wx.z * land_at(b + ivec2(1, dy)) + wx.w * land_at(b + ivec2(2, dy));
}

// Bicubic sample at map point p (cells; cell centres sit at i + 0.5).
vec4 land_smooth(vec2 p) {
    vec2 q = p - 0.5;
    vec2 b = floor(q);
    vec2 f = q - b;
    ivec2 ib = ivec2(b);
    vec4 wx = cr(f.x);
    vec4 wy = cr(f.y);
    return wy.x * land_row(ib, -1, wx) + wy.y * land_row(ib, 0, wx)
        + wy.z * land_row(ib, 1, wx) + wy.w * land_row(ib, 2, wx);
}

vec4 cover_smooth(vec2 p) {
    vec2 q = p - 0.5;
    vec2 b = floor(q);
    vec2 f = smoothstep(0.0, 1.0, q - b);
    ivec2 ib = ivec2(b);
    return mix(
        mix(cover_at(ib), cover_at(ib + ivec2(1, 0)), f.x),
        mix(cover_at(ib + ivec2(0, 1)), cover_at(ib + ivec2(1, 1)), f.x),
        f.y);
}

vec4 plot_smooth(vec2 p) {
    vec2 q = p - 0.5;
    vec2 b = floor(q);
    vec2 f = smoothstep(0.0, 1.0, q - b);
    ivec2 ib = ivec2(b);
    return mix(
        mix(plot_at(ib), plot_at(ib + ivec2(1, 0)), f.x),
        mix(plot_at(ib + ivec2(0, 1)), plot_at(ib + ivec2(1, 1)), f.x),
        f.y);
}

// Integer hash, so the pattern is the same on every GPU.
uint hash_u(uvec3 v) {
    v = v * 1664525u + 1013904223u;
    v.x += v.y * v.z; v.y += v.z * v.x; v.z += v.x * v.y;
    v ^= v >> 16u;
    v.x += v.y * v.z; v.y += v.z * v.x; v.z += v.x * v.y;
    return v.x ^ v.y ^ v.z;
}

float hash(ivec2 c, int k) {
    return float(hash_u(uvec3(ivec3(c, k))) & 0xffffffu) / 16777216.0;
}

float vnoise(vec2 p, int k) {
    ivec2 i = ivec2(floor(p));
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash(i, k), hash(i + ivec2(1, 0), k), f.x),
        mix(hash(i + ivec2(0, 1), k), hash(i + ivec2(1, 1), k), f.x),
        f.y);
}

float fbm(vec2 p, int k) {
    return 0.5 * vnoise(p, k) + 0.3 * vnoise(p * 2.03, k + 1) + 0.2 * vnoise(p * 4.11, k + 2);
}

// Width of one screen pixel in the units of `v`.
float aa(float v) {
    return max(fwidth(v), 1e-4);
}

vec3 sea_color(vec4 s, float coast) {
    float off = -coast;
    float depth = clamp(-s.r / 120.0, 0.0, 1.0);
    vec3 shallow = vec3(0.60, 0.75, 0.80);
    vec3 deep = vec3(0.24, 0.40, 0.58);
    vec3 c = mix(shallow, deep, clamp(smoothstep(0.0, 3.5, off) * 0.7 + depth * 0.5, 0.0, 1.0));
    // Engraved shore lines, the old way of drawing shallows.
    if (u_px > 2.5) {
        float fade = smoothstep(2.5, 6.0, u_px);
        float w = aa(off) * 0.7;
        for (int k = 1; k <= 3; k++) {
            float at = 0.22 * pow(float(k), 1.45);
            float line = 1.0 - smoothstep(0.0, w * 1.6, abs(off - at));
            c = mix(c, vec3(0.30, 0.45, 0.58), line * fade * (0.55 - 0.12 * float(k)));
        }
    }
    return c;
}

// Crop, second crop and fallow strip colours by season (spring, summer, autumn, winter).
const vec3 FIELDS[12] = vec3[12](
    vec3(132.0, 176.0, 86.0) / 255.0, vec3(118.0, 160.0, 76.0) / 255.0, vec3(150.0, 186.0, 106.0) / 255.0,
    vec3(226.0, 196.0, 98.0) / 255.0, vec3(206.0, 172.0, 80.0) / 255.0, vec3(160.0, 178.0, 110.0) / 255.0,
    vec3(186.0, 152.0, 98.0) / 255.0, vec3(170.0, 136.0, 86.0) / 255.0, vec3(150.0, 160.0, 100.0) / 255.0,
    vec3(140.0, 118.0, 92.0) / 255.0, vec3(126.0, 104.0, 80.0) / 255.0, vec3(136.0, 142.0, 108.0) / 255.0);
const vec3 HEDGE = vec3(44.0, 92.0, 40.0) / 255.0;

// @include country.frag

vec3 land_color(vec4 s, vec2 p, float coast) {
    float e = s.r;
    float y = clamp(s.b, 0.0, 1.0);
    float detail = smoothstep(3.0, 14.0, u_px);
    vec4 cv = cover_smooth(p);

    vec3 poor = vec3(0.80, 0.76, 0.60);
    vec3 good = vec3(0.64, 0.74, 0.45);
    vec3 c = mix(poor, good, smoothstep(0.1, 0.9, y));
    // Hypsometric tint: upland browns, bare rock, snow on the highest ground.
    c = mix(c, vec3(0.70, 0.64, 0.50), smoothstep(250.0, 700.0, e) * 0.55);
    c = mix(c, vec3(0.62, 0.60, 0.57), smoothstep(850.0, 1150.0, e));
    c = mix(c, vec3(0.95, 0.95, 0.97), smoothstep(1250.0, 1500.0, e));
    // Ground mottling so open land is not a flat wash.
    c *= 0.95 + 0.10 * fbm(p * 2.3, 11) * mix(0.5, 1.0, detail);
    // A thin strand along the shore.
    c = mix(c, vec3(0.86, 0.81, 0.64), (1.0 - smoothstep(0.0, 0.12, coast)) * 0.6);

    vec2 pm = p * u_cell_m;
    vec4 gr = ground_smooth(p);
    float upland = smoothstep(150.0, 400.0, e);
    // Wetness, upland, poor land: what the trees and grass follow.
    vec4 env = vec4(gr.r, upland, 1.0 - y, 0.0);
    // Close in, the rough grass of open land varies at three scales, as turf does.
    c = turf(c, pm);

    // Pasture (the model's grazed land): downland turf, with tussock marks far out.
    float past = 0.0;
    if (u_show.z > 0.5) {
        float m = cv.b + (fbm(p * 2.5, 61) - 0.5) * 0.5;
        past = smoothstep(0.45, 0.45 + max(aa(m), 0.04 * (1.0 - fine(60.0)) + 0.006), m);
        c = downland(c, p, pm, past * 0.85, upland);
    }

    // The fields: far out, one smooth outline round each block of farmed cells
    // in the season's mean crop colour; closer in, furlongs of strips, meadows,
    // closes and commons, parcel by parcel, with their hedges.
    float fm = cv.g + (fbm(p * 2.2, 71) - 0.5) * 0.45;
    float field = smoothstep(0.5 - max(aa(fm), 0.02), 0.5 + max(aa(fm), 0.02), fm);
    float mosaic = smoothstep(2.0, 5.0, PARCEL / mpx());
    if (u_show.x > 0.5 || u_show.y > 0.5) {
        int base = u_field_season * 3;
        vec3 avg = (FIELDS[base] + FIELDS[base + 1] + FIELDS[base + 2]) / 3.0;
        c = mix(c, avg, field * 0.75 * (1.0 - mosaic));
    }
    // Within a place's edge (R11) the land is its own closes; the fields
    // begin at its edge.
    float inside = u_town_show.x > 0.5 && mosaic > 0.0 ? town_land(pm) : -1e9;
    if (mosaic > 0.0 && (fm > 0.15 || cv.a > 0.02 || inside > -40.0)) {
        c = farmland(c, p, pm, mosaic, upland);
    }

    // Forest: the cover is smooth, its edge broken by noise so woods have ragged
    // margins, more finely the closer in. Far out it is a plain tint; then a
    // texture of stands and groups; close in, single crowns with their shadows,
    // and single trees beyond the wood's edge.
    float f = cv.r;
    float n = clamp((fbm(p * 1.3, 21) - 0.5) * 2.4 + 0.5, 0.0, 1.0);
    float edge = f - n + (vnoise(pm / 90.0, 25) - 0.5) * 0.16 * fine(30.0)
        + (vnoise(pm / 25.0, 27) - 0.5) * 0.06 * fine(8.0);
    float clumps = smoothstep(0.0, max(aa(edge), 0.02), edge);
    float wood = mix(smoothstep(0.05, 0.85, f), clumps, smoothstep(5.0, 12.0, u_px));
    float wood_base = wood;
    // Woods give way to a place's plots and gardens (R11), their edge broken
    // as a wood's edge is.
    if (u_town_show.x > 0.5 && mpx() < 15.0) {
        float ins = town_land(pm) + (vnoise(pm / 25.0, 29) - 0.5) * 30.0;
        wood *= 1.0 - smoothstep(-5.0, 5.0, ins);
        // Trees about the place (R13), as on hand-drawn village maps: groves
        // and single trees among its closes and lots and round its edge, kept
        // off the houses and their yards.
        wood_base = wood;
        float grove = grove_at(pm);
        g_grove_on = grove > 0.0 || town_land(pm) > -60.0;
        wood = max(wood, grove);
    }
    vec3 green = mix(vec3(0.27, 0.45, 0.24), vec3(0.20, 0.37, 0.21), smoothstep(0.4, 0.9, f));
    green = mix(green, vec3(0.24, 0.36, 0.25), smoothstep(400.0, 1000.0, e));
    if (u_season == 2) {
        green = mix(green, vec3(0.66, 0.42, 0.18), 0.35 * smoothstep(0.3, 0.7, f));
    }
    // Coppice (cover cut below its original by fuel cutting) and old wood.
    float cop = clamp((gr.b - f - 0.08) / 0.25, 0.0, 1.0) * step(0.15, f);
    float old = smoothstep(0.6, 0.9, f) * (1.0 - cop);
    float big = 1.0 + 0.2 * old - 0.15 * cop;
    // No trees on the roads (R23): woods, groves and lone trees stand back
    // from them, by about a crown.
    // Tree density, kept per crown from its value and gradient here. Crowns
    // that would stand on a road, a building or a wall are left out whole
    // (`tree_clear`, R27).
    // Without the groves, which are taken at each tree (R27).
    float dens = wood_base * 0.9 + (1.0 - wood_base) * f * 0.035;
    vec2 dgrad = vec2(dFdx(dens) / dFdx(pm.x), dFdy(dens) / dFdy(pm.y));
    vec3 woodc = green;
    // Trees are drawn in their own metres (R12): the look's tree size scales
    // the crowns and their spacing together.
    float ts = u_look.z;
    float tex = smoothstep(25.0, 12.0, mpx() / ts);
    if (tex > 0.0) {
        woodc = mix(green, canopy_texture(pm, env, 5.0 * big * ts), tex);
    }
    float crownsf = fine(10.0 * ts);
    vec3 bare = c;
    c = mix(c, woodc, wood * 0.92 * (1.0 - crownsf));
    if (crownsf > 0.0) {
        vec3 floorc = u_field_season == 3 ? vec3(0.36, 0.29, 0.22) : vec3(0.16, 0.22, 0.12);
        vec3 cc = bare;
        // No woodland floor where its trees are left out, by roads and
        // buildings (R27).
        float floor_free = smoothstep(8.0 * ts, 20.0 * ts, min(g_bnear, u_way_show.x > 0.5 ? g_road_sd : 1e9));
        g_unit = ts;
        vec2 pt = pm / ts;
        dgrad *= ts;
        if (cop > 0.0) {
            // Underwood cut in coupes on a rotation, so each coupe has its own height.
            float age = coupe(pm);
            cc = draw_crowns(cc, pt, 4.5, wood * cop * mix(0.3, 0.95, age), vec2(0.0),
                0.5 + 1.2 * age, 1.0 + 1.8 * age, 521, vec4(env.xyz, 2.0), mix(floorc, vec3(0.45, 0.38, 0.27), 1.0 - age), wood * cop);
        }
        // The woodland floor shows between crowns only well inside the wood.
        cc = draw_crowns(cc, pt, 11.0, dens * (1.0 - 0.8 * cop), dgrad * (1.0 - 0.8 * cop),
            3.5 * big, 8.0 * big, 511, env, floorc,
            smoothstep(0.55, 0.8, wood) * (1.0 - cop) * floor_free);
        g_grove_on = false;
        g_unit = 1.0;
        c = mix(c, cc, crownsf);
    }

    // Gorse and thorn scattered on the downland, and thinly on poor open land.
    // None within a place's edge (R27).
    float thorn = (past * 0.05 + (1.0 - field) * (1.0 - wood) * (1.0 - y) * 0.015)
        * (u_town_show.x > 0.5 ? 1.0 - smoothstep(-30.0, 0.0, town_land(pm)) : 1.0);
    // Its gradient, so a bush is kept or not as a whole (R27).
    vec2 tgrad = vec2(dFdx(thorn) / dFdx(pm.x), dFdy(thorn) / dFdy(pm.y));
    if (thorn > 0.0 && fine(3.0 * ts) > 0.0) {
        float f3 = fine(3.0 * ts);
        g_unit = ts;
        vec3 bush = draw_crowns(c, pm / ts, 13.0, thorn, tgrad * ts, 1.0, 2.6, 541, vec4(0.0, 0.0, 0.0, 1.0), c, 0.0);
        g_unit = 1.0;
        c = mix(c, bush, f3);
    }

    // Deserted places (R5's ruins), drawn at the size of a village since R11:
    // far out a grey patch fading with the years; close in, the earthworks of
    // the crofts under grass: their banks and the house platforms, lit and
    // shaded like low ridges.
    if (u_town_show.x > 0.5) {
        vec4 tv, tgx, tgy;
        town_smooth(p, tv, tgx, tgy);
        float px = 1.0 / u_px;
        float ruin = smoothstep(-px, px, field_dist(tv.g, tgx.g, tgy.g) - 0.3) * (1.0 - step(0.5, tv.r));
        if (ruin > 0.0) {
            float fresh = 0.25 + 0.55 * tv.a;
            c = mix(c, vec3(0.49, 0.45, 0.41), ruin * fresh * (1.0 - fine(40.0)));
            if (fine(4.0) > 0.0) {
                Parcels E = parcels(pm, 45.0, 801);
                float lit = dot(-E.n, SUN);
                float bank = band(E.edge, -1.0, 2.5);
                float platform = band(max(abs(dot(pm - E.sa, vec2(1.0, 0.0))), abs(dot(pm - E.sa, vec2(0.0, 1.0)))), -6.0, 6.0);
                float shade = 1.0 + 0.18 * lit * bank + 0.08 * platform * (0.5 + 0.5 * fresh);
                c *= mix(1.0, shade, ruin * fine(4.0));
            }
        }
    }

    // Seasons: water standing on the flood meadows in spring, frost and snow in winter.
    if (u_season == 0) {
        float meadow = smoothstep(0.25, 0.75, plot_smooth(p).a + (vnoise(p * 3.0, 91) - 0.5) * 0.3);
        c = mix(c, vec3(0.50, 0.68, 0.88), meadow * 0.25);
    } else if (u_season == 3) {
        float snow = smoothstep(700.0, 800.0, e);
        c = mix(c, vec3(0.94, 0.95, 0.97), 0.22 + 0.45 * snow);
    }
    return c;
}

// ---- Roads and rivers at true width (R9) ----

const int WAY_ROW = 2048;
// Most segments read per index square (MAX_WAYS in relief.rs).
const int WAY_MAX = 160;
const vec3 WATER_DEEP = vec3(0.21, 0.36, 0.47);
const vec3 WATER_EDGE = vec3(0.38, 0.48, 0.45);
const vec3 MUD = vec3(0.40, 0.34, 0.25);
const vec3 REED = vec3(0.47, 0.52, 0.31);
const vec3 QUAY_STONE = vec3(0.58, 0.55, 0.50);

// The nearest line of one family (roads or rivers).
struct Way {
    // Metres to its edge, negative inside.
    float sd;
    // Metres from its centreline, signed: positive to the left going along it.
    float x;
    float hw;
    // Metres along the line from its start.
    float along;
    vec2 dir;
    int kind;
    float busy;
};

vec4 way_texel(sampler2D s, int k) {
    return texelFetch(s, ivec2(k % WAY_ROW, k / WAY_ROW), 0);
}

// @include town.frag

// How far a river swings from its line, metres: fixed wavelengths blended by
// size (a wavelength that followed the width would shift the phase wherever
// the width changes), so brooks wind every hundred metres or so and large
// rivers over several hundred. It dies away near the ends of a line, where
// rivers meet each other or the sea.
float meander(float along, float to_end, float hw) {
    float amp = 2.0 + 0.8 * hw;
    float big = smoothstep(3.0, 20.0, hw);
    float swing = mix(vnoise(vec2(along / 90.0, 0.5), 211), vnoise(vec2(along / 420.0, 0.5), 212), big);
    float ends = smoothstep(0.0, 60.0 + 6.0 * hw, min(along, to_end));
    return amp * ends * (swing * 2.0 - 1.0);
}

// `calm` scales the meanders: 0 where a town has embanked the river.
void ways_near(vec2 pw, float calm, out Way road, out Way river) {
    road = Way(1e9, 0.0, 0.0, 0.0, vec2(1.0, 0.0), -1, 0.0);
    river = road;
    ivec2 sq = clamp(ivec2(floor(pw * u_wsub)), ivec2(0), ivec2(u_grid * u_wsub) - 1);
    vec4 cell = texelFetch(u_wcell, sq, 0);
    int start = int(cell.r);
    int n = min(int(cell.g), WAY_MAX);
    for (int k = 0; k < WAY_MAX; k++) {
        if (k >= n) {
            break;
        }
        int j = start + k;
        int id = int(way_texel(u_widx, j >> 2)[j & 3]);
        vec4 a = way_texel(u_ways, 2 * id);
        vec4 b = way_texel(u_ways, 2 * id + 1);
        vec2 ab = a.zw - a.xy;
        vec2 ap = pw - a.xy;
        float l2 = max(dot(ab, ab), 1e-12);
        float t = clamp(dot(ap, ab) / l2, 0.0, 1.0);
        float d = length(ap - ab * t) * u_cell_m;
        float hw = mix(b.x, b.y, t);
        float sd = d - hw;
        int kind = int(floor(b.z / 4.0));
        float extra = b.z - 4.0 * float(kind);
        if (kind >= 2) {
            if (sd < road.sd) {
                float side = ab.x * ap.y - ab.y * ap.x < 0.0 ? -1.0 : 1.0;
                float len = sqrt(l2);
                road = Way(sd, side * d, hw, b.w + t * len * u_cell_m, ab / len, kind, extra);
            }
        } else if (sd - (2.0 + 0.8 * hw) < river.sd) {
            // Rivers are compared after their meander, which can bring a farther
            // line nearer; only lines within a swing of the nearest are worth it.
            float side = ab.x * ap.y - ab.y * ap.x < 0.0 ? -1.0 : 1.0;
            float len = sqrt(l2);
            float along = b.w + t * len * u_cell_m;
            float to_end = extra * 1000.0 - t * len * u_cell_m;
            float x = side * d - calm * meander(along, extra >= 0.998 ? 1e9 : to_end, hw);
            float msd = abs(x) - hw;
            if (msd < river.sd) {
                river = Way(msd, x, hw, along, ab / len, kind, 0.0);
            }
        }
    }
}

// Roads and rivers drawn into the land colour at their real width, at map point
// `p` with the smoothed cell values `s` (its alpha is the hillshade).
// `town` is how far into a built-up area, `rank` the place's (from the town
// scan; 0 when the settlements layer is off), `share` how much of the ground
// round about is built over.
// `msd` is the metres to a market place's edge, negative inside (R22): it is
// paved as the streets are.
// The nearest road and river to map point p. Their bends are laid out on the
// CPU (R24); here they only wander a metre or two, so edges are not ruled.
void ways_at(vec2 p, float share, out Way road, out Way river) {
    vec2 pm = p * u_cell_m;
    vec2 wander = (vec2(vnoise(pm / 60.0, 205), vnoise(pm / 60.0, 207)) - 0.5) * 2.0;
    ways_near(p + wander / u_cell_m, 1.0 - smoothstep(0.0, 0.08, share), road, river);
}

float road_sd_at(vec2 pm) {
    Way road, river;
    ways_near(pm / u_cell_m, 1.0, road, river);
    return road.sd;
}

vec3 ways_color(vec3 c, vec2 p, vec4 s, float town, float rank, float share, float msd, Way road, Way river) {
    float shade = s.a;
    vec2 pm = p * u_cell_m;
    if (min(road.sd / u_look.x, river.sd) > 40.0 && msd > 2.0) {
        return c;
    }
    float wood = smoothstep(0.4, 0.8, cover_smooth(p).r);
    bool winter = u_season == 3;
    bool wet = u_season == 0 || winter;

    // Rivers: open water shading from the margins to the deeper middle, a muddy
    // bank, reeds in patches, quays in town; swinging in meanders sized to the river.
    float rcov = 0.0;
    float rd = 1e9;
    float rhw = 0.0;
    if (u_way_show.y > 0.5 && river.sd < 40.0) {
        rhw = river.hw * u_way_show.z;
        float x = river.x;
        rd = abs(x);
        if (town > 0.5 && rank > 1.5 && u_town_show.y > 0.5) {
            c = mix(c, QUAY_STONE * (0.9 + 0.2 * vnoise(pm / 1.2, 219) * fine(1.2)), band(rd, rhw, rhw + 1.6));
            c = mix(c, vec3(0.30, 0.27, 0.24), band(rd, rhw + 1.6, rhw + 1.9));
        } else {
            float patches = smoothstep(0.35, 0.7, vnoise(vec2(river.along / 35.0, sign(x) * 3.0), 213));
            float reach = (1.0 + 0.12 * rhw) * (0.3 + patches) * (1.0 - 0.6 * wood);
            vec3 reed = REED * (0.85 + 0.3 * vnoise(pm / 0.7, 215) * fine(0.7));
            if (u_season == 2 || winter) {
                reed = mix(reed, vec3(0.62, 0.55, 0.38), 0.6);
            }
            c = mix(c, reed, band(rd, rhw, rhw + 0.5 + reach) * 0.85);
            c = mix(c, MUD, band(rd, rhw - 0.2, rhw + 0.3 + 0.03 * rhw) * 0.8);
        }
        float depth = clamp(1.0 - rd / max(rhw, 0.1), 0.0, 1.0);
        vec3 water = mix(WATER_EDGE, WATER_DEEP, smoothstep(0.0, 0.7, depth) * clamp(rhw / 12.0, 0.35, 1.0));
        // Streaks drawn out along the current.
        float streak = vnoise(vec2(river.along / 9.0, x / 1.4), 217);
        water *= 0.95 + 0.1 * streak * fine(1.4);
        if (winter) {
            water = mix(water, vec3(0.80, 0.85, 0.88), 0.5 * (1.0 - smoothstep(0.0, 3.0, rhw - rd)));
        }
        rcov = band(rd, -rhw, rhw);
        c = mix(c, water, rcov);
        // The bank in ink.
        float iw = max(0.3, 1.3 * mpx());
        c = mix(c, vec3(0.16, 0.21, 0.25), band(rd, rhw - iw, rhw + 0.3 * iw) * u_ink);
    }

    // Roads are drawn in their own metres (R12): a road the look widens has
    // its ruts, verges, ditches and hedges widened with it.
    float rsc = u_look.x;
    g_unit = rsc;
    road.sd /= rsc;
    road.x /= rsc;
    road.hw /= rsc;
    road.along /= rsc;
    pm /= rsc;
    rd /= rsc;
    rhw /= rsc;

    // A market place (R22): the town street's own surface, without ruts,
    // with its edge in ink as a street's is.
    if (u_way_show.x > 0.5 && msd < 2.0) {
        float md = msd / rsc;
        float mcov = clamp(0.5 - md / mpx(), 0.0, 1.0) * (1.0 - rcov);
        float mbusy = road.sd < 40.0 ? road.busy : 0.0;
        vec3 soil = mix(vec3(0.68, 0.57, 0.42), vec3(0.52, 0.42, 0.30), clamp(s.b, 0.0, 1.0));
        vec3 surf = mix(soil, vec3(0.78, 0.69, 0.52), 0.45 * mbusy);
        surf *= 0.92 + 0.16 * vnoise(pm / 2.5, 223) * fine(2.5);
        if (wet) {
            float pud = smoothstep(0.66, 0.74, vnoise(pm / 1.7, 227));
            surf = mix(surf, vec3(0.55, 0.61, 0.66), pud * 0.85);
        }
        if (winter) {
            surf = mix(surf, vec3(0.90, 0.91, 0.93), 0.4);
        }
        vec3 street = vec3(0.64, 0.61, 0.56) * (0.9 + 0.2 * vnoise(pm / 0.6, 237) * fine(0.6));
        surf = mix(surf, street, 0.8 * smoothstep(1.2, 2.0, max(rank, 2.0)));
        c = mix(c, surf, mcov);
    }
    // Inside the market place the roads lose their edges (R23): the place is
    // the road itself, and its own edge is drawn after them.
    float in_market = clamp(0.5 - (msd / rsc) / mpx(), 0.0, 1.0) * (1.0 - rcov);
    float roadcov = 0.0;
    vec3 c_market = c;

    // Roads: a worn surface the colour of the soil, ruts, grass down the middle of
    // quiet lanes, worn verges, banks where a lane cuts into a slope, and ditches
    // and hedges along the highways; paved streets in town; bridges over water.
    if (road.kind == 4) {
        // A footpath: a trodden line, wandering a little, narrower in the grass.
        if (u_way_show.w > 0.5 && road.sd < 10.0) {
            float hw = road.hw * (0.75 + 0.5 * vnoise(vec2(road.along / 8.0, 0.5), 251));
            float d = abs(road.x + 0.6 * (vnoise(vec2(road.along / 25.0, 1.5), 253) - 0.5));
            vec3 trod = mix(vec3(0.62, 0.55, 0.42), c, 0.35) * (0.92 + 0.16 * vnoise(pm / 0.8, 255) * fine(0.8));
            if (winter) {
                trod = mix(trod, vec3(0.42, 0.36, 0.30), 0.4);
            }
            float over = rcov > 0.0 ? band(rd, -rhw - 1.0, rhw + 1.0) : 0.0;
            trod = mix(trod, vec3(0.52, 0.40, 0.28), over);
            // Paths give way to the plots and streets of a place.
            c = mix(c, trod, band(d, -hw, hw) * (1.0 - max(town, smoothstep(0.05, 0.15, share))));
        }
    } else if (u_way_show.x > 0.5 && road.sd < 40.0) {
        // The town plans' own streets are town streets throughout.
        if (road.kind == 5) {
            town = 1.0;
        }
        // Land trade off the roads gives way to a town's own streets.
        if (road.kind == 2 && rank > 1.5 && max(town, step(0.1, share)) > 0.0) {
            g_unit = 1.0;
            return c;
        }
        float busy = road.busy;
        bool track = road.kind == 2;
        float level = 1.0 - smoothstep(0.06, 0.18, abs(shade - 1.0));
        // Carts spread round the mud on low, flat ground.
        float spread = 1.0 + 0.45 * level * smoothstep(0.55, 0.85, vnoise(vec2(road.along / 70.0, 0.5), 221));
        float hw = road.hw * (town > 0.5 ? 1.0 : spread);
        float x = road.x;
        float d = abs(x);
        float side = sign(x);
        vec3 soil = mix(vec3(0.68, 0.57, 0.42), vec3(0.52, 0.42, 0.30), clamp(s.b, 0.0, 1.0));
        vec3 surf = mix(soil, vec3(0.78, 0.69, 0.52), 0.45 * busy);
        surf *= 0.92 + 0.16 * vnoise(pm / 2.5, 223) * fine(2.5);
        // Wheel ruts 1.4 m apart, one pair to a lane, worn deeper on busy roads.
        bool two = hw > 2.6;
        float lx = two ? abs(d - 0.5 * hw) : d;
        float rut = band(lx, 0.5, 0.9) * (0.4 + 0.6 * vnoise(vec2(road.along / 11.0, side), 225));
        float mid = two ? 0.0 : band(lx, -0.35, 0.35) * (1.0 - smoothstep(0.1, 0.4, busy));
        surf = mix(surf, mix(c, vec3(0.50, 0.56, 0.33), 0.4), mid * 0.65 * (1.0 - town));
        if (wet) {
            float pud = smoothstep(0.66, 0.74, vnoise(pm / 1.7, 227)) * (1.0 - smoothstep(0.7, 1.0, d / hw));
            surf = mix(surf, vec3(0.55, 0.61, 0.66), pud * 0.85);
        }
        if (winter) {
            surf = mix(surf, vec3(0.90, 0.91, 0.93), 0.4);
        }
        surf *= 1.0 - (0.22 + 0.15 * busy) * rut;
        if (town > 0.0) {
            // Paved or cobbled through the town.
            vec3 street = vec3(0.64, 0.61, 0.56) * (0.9 + 0.2 * vnoise(pm / 0.6, 237) * fine(0.6));
            surf = mix(surf, street, town * 0.8 * smoothstep(1.2, 2.0, rank));
        }
        float open = 1.0 - town;
        // Verges: grass kept short by passing stock, worn bare next to the road.
        float vw = (0.8 + 2.4 * busy) * (0.75 + 0.5 * vnoise(vec2(road.along / 14.0, side * 2.0), 229));
        vec3 grass = vec3(0.50, 0.60, 0.33) * (0.92 + 0.16 * vnoise(pm / 1.3, 241) * fine(1.3));
        if (u_field_season == 2) {
            grass = mix(grass, vec3(0.60, 0.57, 0.36), 0.35);
        } else if (u_field_season == 3) {
            grass = mix(grass, vec3(0.60, 0.60, 0.50), 0.6);
        }
        c = mix(c, grass, band(d, hw, hw + vw) * open * (track ? 0.4 : 0.85));
        c = mix(c, mix(soil, grass, 0.5), band(d, hw, hw + 0.4 + 0.6 * busy) * open);
        // A lane worn into a slope: the bank facing the light is lit, the other shaded.
        float sunk = (1.0 - level) * (1.0 - busy) * open;
        vec2 nrm = vec2(-road.dir.y, road.dir.x) * side;
        float lit = dot(-nrm, vec2(-0.7071, -0.7071));
        c *= 1.0 + 0.4 * lit * sunk * band(d, hw, hw + 1.8);
        if (busy > 0.5 && !track) {
            // Highways: a ditch each side, and a hedge beyond it, broken by gates.
            // None within a place's edge (R27), where they broke into
            // fragments between the plots.
            float in_town = u_town_show.x > 0.5 ? smoothstep(-30.0, 0.0, town_land(p * u_cell_m)) : 0.0;
            float hi = smoothstep(0.5, 0.8, busy) * open * (1.0 - in_town);
            float dd = hw + vw;
            vec3 ditch = wet ? vec3(0.33, 0.41, 0.46) : vec3(0.27, 0.31, 0.19);
            c = mix(c, ditch, band(d, dd, dd + 1.0) * hi);
            float hz = dd + 1.2;
            float gate = smoothstep(0.1, 0.18, vnoise(vec2(road.along / 45.0, side * 7.0), 231));
            float hwid = 1.8 + 1.0 * vnoise(vec2(road.along / 7.0, side * 3.0), 233)
                + 0.6 * (vnoise(vec2(road.along / 2.2, side * 5.0), 243) - 0.5);
            float hedge_on = hi * gate * (1.0 - wood);
            // The hedge's shadow, about its height, on the side away from the light.
            float shadow = dot(nrm, vec2(-0.7071, -0.7071)) > 0.0
                ? band(d, hz - 1.6, hz) : band(d, hz + hwid, hz + hwid + 1.6);
            c *= 1.0 - 0.3 * shadow * hedge_on;
            vec3 hedge = HEDGE * (0.8 + 0.45 * vnoise(pm / 1.1, 235) * fine(1.1));
            c = mix(c, hedge, band(d, hz, hz + hwid) * hedge_on);
        }
        // Over water the road is a bridge: stone for busy roads, timber otherwise,
        // with parapets along its edges.
        float over = rcov > 0.0 ? band(rd, -rhw - 2.0, rhw + 2.0) : 0.0;
        if (over > 0.0) {
            vec3 deck = busy > 0.3 ? vec3(0.66, 0.62, 0.55) : vec3(0.52, 0.40, 0.28);
            surf = mix(surf, deck * (0.93 + 0.14 * vnoise(vec2(road.along / 0.5, x), 239) * fine(0.5)), over);
            float hwb = hw + 0.5;
            c = mix(c, vec3(0.30, 0.27, 0.24), band(d, hw, hwb) * over);
            hw = mix(hw, hwb, over);
        }
        c = mix(c, surf, band(d, -hw, hw));
        roadcov = band(d, -hw, hw);
        // A faint edge where the surface meets the grass.
        c = mix(c, c * 0.86, band(d, hw - 0.3, hw + 0.1) * open * (1.0 - over) * (1.0 - in_market));
        // Its edges in ink, lighter than a building's.
        float iw = max(0.25, 1.0 * mpx());
        c = mix(c, mix(c, INK, 0.6), band(d, hw - iw, hw) * u_ink * (1.0 - in_market));
    }
    // Within it the roads are the place's own surface.
    c = mix(c, c_market, in_market);
    // The market place's edge in ink, as a street's, broken where the roads
    // leave it (R23).
    if (msd < 2.0) {
        float iw = max(0.25, 1.0 * mpx());
        c = mix(c, mix(c, INK, 0.6), band(-msd / rsc, 0.0, iw) * u_ink * (1.0 - roadcov));
    }
    g_unit = 1.0;
    return c;
}

void main() {
    vec2 p = mix(u_p0, u_p1, v_uv);
    if (p.x < 0.0 || p.y < 0.0 || p.x >= u_grid.x || p.y >= u_grid.y) {
        discard;
    }
    vec4 s = land_smooth(p);
    // The coast wanders a little within its cell so it does not read as a curve fit.
    float coast = s.g + (vnoise(p * 3.0, 3) - 0.5) * 0.30 + (vnoise(p * 9.0, 5) - 0.5) * 0.08;
    float shade = clamp(s.a, 0.45, 1.45);

    vec3 sea = sea_color(s, coast);
    vec2 pm = p * u_cell_m;
    // Towns close in: the ground of their plots, then the streets, then the
    // shadows and roofs; far out, the share of the ground built over, in the
    // colour of roofs.
    bool close_in = mpx() < 15.0;
    Town T = Town(-1, vec2(0.0), vec2(1.0), 0.0, 0.0, 0.0,
                  -1, vec2(0.0), vec2(1.0), 0.0, 0.0, 0.0, -1.0, 0.0,
                  0.0, 0.0, 0.0, 1e9, 1e9);
    if (u_town_show.x > 0.5 && close_in) {
        T = town_scan(pm);
        g_bnear = T.bnear;
    }
    // The roads and rivers, found once, before the land so trees keep off the
    // roads (R23).
    bool ways_on = (u_way_show.x > 0.5 || u_way_show.y > 0.5 || u_way_show.w > 0.5) && close_in;
    float share = u_town_show.x > 0.5 && close_in ? built_share(pm) : 0.0;
    Way road = Way(1e9, 0.0, 0.0, 0.0, vec2(1.0, 0.0), -1, 0.0);
    Way river = road;
    if (ways_on) {
        ways_at(p, share, road, river);
        g_road_sd = road.sd;
    }
    // The pixel's position, for trees deciding whether to look further.
    g_pm = pm;
    vec3 land = land_color(s, p, coast);
    if (u_town_show.x > 0.5 && close_in) {
        land = town_ground(land, pm, T);
    }
    // Roads and rivers at true width, once a pixel is under 15 m: past the old
    // deepest zoom, where the map symbols begin to fade. Further out they are
    // a fraction of a pixel and the symbols carry them.
    if (ways_on) {
        land = ways_color(land, p, s, T.near, T.rank, share, T.msd, road, river);
    }
    if (u_town_show.x > 0.5) {
        if (close_in) {
            land *= 1.0 - 0.35 * T.shadow * fine(1.0);
            land = town_buildings(land, T);
        }
        float far = smoothstep(4.0, 15.0, mpx());
        if (far > 0.0) {
            float share = built_share(pm);
            land = mix(land, vec3(0.58, 0.40, 0.31), clamp(share * 1.4, 0.0, 0.9) * far);
        }
    }
    land *= shade;
    float w = aa(coast);
    vec3 c = mix(sea, land, smoothstep(-w, w, coast));
    // Coastline in ink once a cell is a few pixels across.
    float ink = (1.0 - smoothstep(0.0, w * 1.3, abs(coast))) * smoothstep(1.5, 4.0, u_px);
    c = mix(c, vec3(0.20, 0.26, 0.30), ink * 0.85);
    // Quays: where a town's plots come down to the sea, the shore is a timber
    // and stone edge.
    if (u_town_show.y > 0.5 && close_in && abs(coast) < 0.4 && T.rank > 1.5 && T.near > 0.0) {
        float px = 1.0 / u_px;
        float qw = max(0.006, 2.5 * px);
        float quay = 1.0 - smoothstep(qw - px, qw, abs(coast - qw * 0.5));
        float edge = 1.0 - smoothstep(0.0, px, abs(coast));
        c = mix(c, vec3(92.0, 64.0, 40.0) / 255.0, quay * T.near);
        c = mix(c, vec3(0.22, 0.16, 0.10), edge * T.near);
    }
    o_color = vec4(c, 1.0);
}
