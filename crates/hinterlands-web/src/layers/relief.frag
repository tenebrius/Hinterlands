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
// 1 navigable river, 2 track, 3 road. On rivers, busy is replaced by the
// kilometres to the end of the line (up to 0.999).
uniform sampler2D u_ways;
// Per square of the index grid (u_wsub squares to a cell along a side): first
// entry in u_widx and how many segments reach into the square.
uniform sampler2D u_wcell;
uniform float u_wsub;
// Segment numbers, four to a texel.
uniform sampler2D u_widx;
// True-width roads on, rivers on, spring widening of the rivers.
uniform vec3 u_way_show;
// Metres per cell.
uniform float u_cell_m;

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

// Tree crowns: distance to the nearest crown centre (in crown units) and that crown's id.
vec2 crowns(vec2 q, int k, out ivec2 id) {
    ivec2 b = ivec2(floor(q));
    float best = 9.0;
    float r = 0.0;
    id = b;
    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            ivec2 c = b + ivec2(dx, dy);
            vec2 o = vec2(hash(c, k), hash(c, k + 7)) * 0.7 + 0.15;
            float d = length(vec2(c) + o - q);
            if (d < best) {
                best = d;
                id = c;
                r = 0.38 + 0.22 * hash(c, k + 13);
            }
        }
    }
    return vec2(best, r);
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

// Field boundaries follow the cells, but wander so furlongs are not squares.
vec2 warp(vec2 p) {
    return p + (vec2(vnoise(p * 2.0, 51), vnoise(p * 2.0, 53)) - 0.5) * 0.35;
}

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

    // Pasture beyond the fields: a meadow green with ragged edges.
    if (u_show.z > 0.5) {
        float m = cv.b + (fbm(p * 2.5, 61) - 0.5) * 0.5;
        float past = smoothstep(0.45, 0.45 + max(aa(m), 0.04), m);
        vec3 grass = u_field_season == 3 ? vec3(0.60, 0.60, 0.50) : vec3(0.60, 0.73, 0.40);
        grass *= 0.94 + 0.12 * vnoise(p * 14.0, 63) * detail;
        c = mix(c, grass, past * 0.75);
    }

    // The fields: one smooth outline around each block of farmed cells.
    float fm = cv.g + (fbm(p * 2.2, 71) - 0.5) * 0.45;
    float field = smoothstep(0.5 - max(aa(fm), 0.02), 0.5 + max(aa(fm), 0.02), fm);
    vec2 q = warp(p);
    vec4 pl = plot_at(ivec2(floor(q)));

    // Land that has gone out of cultivation keeps the faint lines of its strips.
    if (u_show.w > 0.5) {
        float gm = smoothstep(0.02, 0.15, cv.a) * (1.0 - field);
        float s2 = q.y * 4.0;
        float e2 = min(fract(s2), 1.0 - fract(s2));
        float line = 1.0 - smoothstep(0.0, aa(s2) * 1.2, e2);
        c = mix(c, vec3(0.38, 0.33, 0.24), line * gm * (0.15 + 0.4 * cv.a) * smoothstep(6.0, 10.0, u_px));
    }

    float show = max(u_show.x, u_show.y * smoothstep(5.0, 7.0, u_px));
    if (show > 0.0) {
        // Strips run away from the nearest road or river, with the reversed S of the plough team,
        // and one in three lies fallow, moving on each year.
        float g = pl.g;
        bool horiz = mod(g, 2.0) > 0.5;
        float rot = floor(g / 2.0 + 0.01);
        float along = horiz ? q.x : q.y;
        float across = (horiz ? q.y : q.x) + 0.03 * sin(along * 6.2832);
        float sk = across * 6.0;
        float k = floor(sk);
        int base = u_field_season * 3;
        vec3 crop = mod(k, 2.0) < 0.5 ? FIELDS[base] : FIELDS[base + 1];
        vec3 strip = mod(k + u_year + rot, 3.0) < 0.5 ? FIELDS[base + 2] : crop;
        float fs = min(fract(sk), 1.0 - fract(sk));
        strip *= 1.0 - 0.16 * (1.0 - smoothstep(0.0, aa(sk) * 1.2, fs));
        strip *= 0.95 + 0.10 * vnoise(vec2(along * 24.0, k * 3.1), 73);
        vec3 avg = (FIELDS[base] + FIELDS[base + 1]) / 3.0 + FIELDS[base + 2] / 3.0;
        float stripes = smoothstep(10.0, 22.0, u_px) * u_show.y;
        c = mix(c, mix(avg, strip, stripes), field * show * mix(0.75, 1.0, stripes));
    }

    // Forest: the cover is smooth, its edge broken by noise so woods have ragged margins.
    // Far out that would be speckle, so cover is a plain tint until cells are a few pixels wide.
    float f = cv.r;
    float n = clamp((fbm(p * 1.3, 21) - 0.5) * 2.4 + 0.5, 0.0, 1.0);
    float edge = f - n;
    float clumps = smoothstep(0.0, max(aa(edge), 0.05), edge);
    float wood = mix(smoothstep(0.05, 0.85, f), clumps, smoothstep(5.0, 12.0, u_px));
    vec3 green = mix(vec3(0.27, 0.45, 0.24), vec3(0.20, 0.37, 0.21), smoothstep(0.4, 0.9, f));
    green = mix(green, vec3(0.24, 0.36, 0.25), smoothstep(400.0, 1000.0, e));
    if (u_season == 2) {
        green = mix(green, vec3(0.66, 0.42, 0.18), 0.35 * smoothstep(0.3, 0.7, f));
    }
    if (detail > 0.0) {
        // Close up the wood resolves into crowns; single trees stand out past its edge.
        ivec2 id;
        vec2 cw = crowns(p * 4.0, 31, id);
        float inside = 1.0 - smoothstep(cw.y - aa(cw.x), cw.y, cw.x);
        float lit = clamp(1.0 - cw.x / cw.y, 0.0, 1.0);
        vec3 crown = green * (0.80 + 0.40 * lit);
        vec3 gap = green * 0.62;
        vec3 canopy = mix(gap, crown, inside);
        green = mix(green, canopy, detail);
        float stray = step(hash(id, 41), f * 0.6) * inside * (1.0 - wood);
        c = mix(c, crown, stray * detail * 0.9);
    }
    c = mix(c, green, wood * 0.92);

    // Hedgerows round fields farmed for fifty years: on the outline of the field block
    // and between neighbouring holdings, thickening and thinning like real hedges.
    if (u_show.w > 0.5 && u_px > 6.0) {
        float hw = max(0.05, 1.0 / u_px);
        float hedged = plot_smooth(p).b;
        float grad = length(vec2(dFdx(fm), dFdy(fm))) * u_px;
        float rim = abs(fm - 0.5) / max(grad, 1e-3);
        float on = (1.0 - smoothstep(hw * 0.6, hw, rim)) * step(0.3, hedged);
        if (pl.r >= 0.0 && pl.b > 0.5) {
            vec2 off[4] = vec2[4](vec2(hw, 0.0), vec2(-hw, 0.0), vec2(0.0, hw), vec2(0.0, -hw));
            for (int k = 0; k < 4; k++) {
                vec4 o = plot_at(ivec2(floor(warp(p + off[k]))));
                if (o.r >= 0.0 && abs(o.r - pl.r) > 1e-4) {
                    on = max(on, field);
                }
            }
        }
        float bushes = 0.55 + 0.45 * smoothstep(0.3, 0.6, vnoise(p * 28.0, 81));
        c = mix(c, HEDGE * (0.85 + 0.3 * vnoise(p * 60.0, 83)), on * bushes * smoothstep(6.0, 10.0, u_px));
    }

    // Settlements: one smooth outline round each built-up area, ruins fading grey,
    // and close up the streets: house plots along the lanes, gardens behind,
    // thinning towards the edge of town. Roads and rivers are drawn over them.
    if (u_town_show.x > 0.5) {
        vec4 tv, tgx, tgy;
        town_smooth(p, tv, tgx, tgy);
        float px = 1.0 / u_px;
        float db = field_dist(tv.r, tgx.r, tgy.r);
        float inside = smoothstep(-px, px, db);
        float ruin = smoothstep(-px, px, field_dist(tv.g, tgx.g, tgy.g)) * (1.0 - inside);
        vec3 rubble = vec3(0.49, 0.45, 0.41);
        if (u_px > 12.0) {
            // Broken walls of the old houses.
            vec2 rq = fract(p * 9.0);
            float wallv = min(min(rq.x, 1.0 - rq.x), min(rq.y, 1.0 - rq.y));
            float broken = step(0.45, vnoise(p * 9.0, 111));
            rubble = mix(rubble, vec3(0.36, 0.33, 0.30), (1.0 - smoothstep(0.04, 0.09, wallv)) * broken);
        }
        c = mix(c, rubble, ruin * (0.25 + 0.55 * tv.a));

        vec3 town = mix(vec3(178.0, 74.0, 58.0) / 255.0, vec3(196.0, 160.0, 120.0) / 255.0, smoothstep(3.0, 6.0, u_px));
        float streets = smoothstep(10.0, 18.0, u_px);
        if (streets > 0.0) {
            // Blocks of a third of a cell, the street grid bent a little.
            vec2 q = p * 3.0 + (vec2(vnoise(p * 1.5, 101), vnoise(p * 1.5, 103)) - 0.5) * 0.6;
            ivec2 blk = ivec2(floor(q));
            vec2 u = fract(q);
            vec2 e = min(u, 1.0 - u);
            float lane = min(e.x, e.y);
            float along = e.x < e.y ? u.y : u.x;
            float plot = floor(along * 7.0);
            int side = e.x < e.y ? (u.x < 0.5 ? 0 : 1) : (u.y < 0.5 ? 2 : 3);
            float h = hash(blk * 4 + ivec2(side, int(plot)), 107);
            float empty = 0.08 + 0.55 * (1.0 - smoothstep(0.0, 0.7, db));
            vec3 street = vec3(0.86, 0.80, 0.68);
            vec3 garden = vec3(0.60, 0.65, 0.43);
            vec3 roof = vec3(0.62, 0.27, 0.19) * (0.82 + 0.3 * hash(blk, 109) + 0.1 * h);
            float a = aa(lane);
            float house = smoothstep(0.07, 0.07 + a, lane) * (1.0 - smoothstep(0.33, 0.33 + a, lane));
            float gap = 1.0 - smoothstep(0.0, 0.08, min(fract(along * 7.0), 1.0 - fract(along * 7.0)));
            house *= step(empty, h) * (1.0 - gap * 0.8);
            vec3 blockc = mix(garden, roof, house);
            blockc = mix(street, blockc, smoothstep(0.07 - a, 0.07, lane));
            town = mix(town, blockc, streets);
        }
        c = mix(c, town, inside);
        float rim = 1.0 - smoothstep(0.0, 1.3 * px, abs(db));
        c = mix(c, vec3(0.42, 0.30, 0.22), rim * smoothstep(3.0, 6.0, u_px) * 0.8);

        // City walls on the outline of the walled area, with towers spaced along them.
        if (u_town_show.y > 0.5 && u_px > 3.0) {
            vec3 stone = vec3(150.0, 140.0, 125.0) / 255.0;
            vec3 dark = vec3(0.30, 0.27, 0.24);
            float dw = field_dist(tv.b, tgx.b, tgy.b);
            float hw = max(0.035, 1.2 * px);
            float wall = 1.0 - smoothstep(hw, hw + px, abs(dw));
            float edgeline = 1.0 - smoothstep(0.0, px, abs(abs(dw) - hw));
            c = mix(c, stone, wall);
            c = mix(c, dark, edgeline * wall * smoothstep(5.0, 10.0, u_px));
            vec2 lp = floor(p * 2.0);
            for (int dy = 0; dy <= 1; dy++) {
                for (int dx = 0; dx <= 1; dx++) {
                    vec2 l = (lp + vec2(float(dx), float(dy))) / 2.0;
                    vec4 lv, lgx, lgy;
                    town_smooth(l, lv, lgx, lgy);
                    vec2 grad = vec2(lgx.b, lgy.b);
                    float g2 = max(dot(grad, grad), 1e-3);
                    float dl = field_dist(lv.b, lgx.b, lgy.b);
                    if (abs(dl) < 0.25 && lv.b > 0.05 && lv.b < 0.95) {
                        vec2 at = l - grad * (lv.b - 0.5) / g2;
                        float rt = max(0.07, 2.5 * px);
                        float r = length(p - at);
                        c = mix(c, stone, 1.0 - smoothstep(rt - px, rt, r));
                        c = mix(c, dark, (1.0 - smoothstep(0.0, px, abs(r - rt + 0.5 * px))) * smoothstep(5.0, 10.0, u_px));
                    }
                }
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
const int WAY_MAX = 96;
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

// Metres per physical pixel.
float mpx() {
    return u_cell_m / u_px;
}

// How much of the band lo..hi metres across a pixel centred at x covers:
// a box filter, so lines thinner than a pixel fade instead of vanishing.
float band(float x, float lo, float hi) {
    float m = mpx();
    return clamp((min(x + 0.5 * m, hi) - max(x - 0.5 * m, lo)) / m, 0.0, 1.0);
}

// Detail of this size in metres, faded out before it is small enough to shimmer.
float fine(float metres) {
    return smoothstep(1.5, 4.0, metres / mpx());
}

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

void ways_near(vec2 pw, out Way road, out Way river) {
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
            float x = side * d - meander(along, extra >= 0.998 ? 1e9 : to_end, hw);
            float msd = abs(x) - hw;
            if (msd < river.sd) {
                river = Way(msd, x, hw, along, ab / len, kind, 0.0);
            }
        }
    }
}

// Roads and rivers drawn into the land colour at their real width, at map point
// `p` with the smoothed cell values `s` (its alpha is the hillshade).
vec3 ways_color(vec3 c, vec2 p, vec4 s) {
    float shade = s.a;
    vec2 pm = p * u_cell_m;
    // Lines wander either side of their smoothed course, some 20 m over most of a
    // kilometre and a few metres over a furlong, so no road runs dead straight
    // along the grid. Roads and rivers wander together, so they stay apart.
    vec2 wander = (vec2(vnoise(pm / 700.0, 201), vnoise(pm / 700.0, 203)) - 0.5) * 40.0
        + (vec2(vnoise(pm / 160.0, 205), vnoise(pm / 160.0, 207)) - 0.5) * 8.0;
    vec2 pw = p + wander / u_cell_m;
    Way road, river;
    ways_near(pw, road, river);
    if (min(road.sd, river.sd) > 40.0) {
        return c;
    }
    // How far inside a built-up area, and how wooded.
    float town = 0.0;
    if (u_town_show.x > 0.5) {
        vec4 tv, tgx, tgy;
        town_smooth(p, tv, tgx, tgy);
        town = smoothstep(-0.02, 0.02, field_dist(tv.r, tgx.r, tgy.r));
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
        if (town > 0.5 && u_town_show.y > 0.5) {
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
    }

    // Roads: a worn surface the colour of the soil, ruts, grass down the middle of
    // quiet lanes, worn verges, banks where a lane cuts into a slope, and ditches
    // and hedges along the highways; paved streets in town; bridges over water.
    if (u_way_show.x > 0.5 && road.sd < 40.0) {
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
            surf = mix(surf, street, town * 0.8);
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
            float hi = smoothstep(0.5, 0.8, busy) * open;
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
        // A faint edge where the surface meets the grass.
        c = mix(c, c * 0.86, band(d, hw - 0.3, hw + 0.1) * open * (1.0 - over));
    }
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
    vec3 land = land_color(s, p, coast);
    // Roads and rivers at true width, once a pixel is under 15 m: past the old
    // deepest zoom, where the map symbols begin to fade. Further out they are
    // a fraction of a pixel and the symbols carry them.
    if ((u_way_show.x > 0.5 || u_way_show.y > 0.5) && mpx() < 15.0) {
        land = ways_color(land, p, s);
    }
    land *= shade;
    float w = aa(coast);
    vec3 c = mix(sea, land, smoothstep(-w, w, coast));
    // Coastline in ink once a cell is a few pixels across.
    float ink = (1.0 - smoothstep(0.0, w * 1.3, abs(coast))) * smoothstep(1.5, 4.0, u_px);
    c = mix(c, vec3(0.20, 0.26, 0.30), ink * 0.85);
    // Quays: where a town's footprint meets the sea, the shore is a timber and stone edge.
    if (u_town_show.y > 0.5 && u_px > 4.0 && abs(coast) < 0.4) {
        vec4 tv, tgx, tgy;
        town_smooth(p, tv, tgx, tgy);
        float near = smoothstep(-0.35, -0.1, field_dist(tv.r, tgx.r, tgy.r));
        float port = near * step(0.5, tv.a / max(tv.r, 1e-3));
        float px = 1.0 / u_px;
        float qw = max(0.06, 2.5 * px);
        float quay = 1.0 - smoothstep(qw - px, qw, abs(coast - qw * 0.5));
        float edge = 1.0 - smoothstep(0.0, px, abs(coast));
        c = mix(c, vec3(92.0, 64.0, 40.0) / 255.0, quay * port);
        c = mix(c, vec3(0.22, 0.16, 0.10), edge * port);
    }
    o_color = vec4(c, 1.0);
}
