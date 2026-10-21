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

in vec2 v_uv;
out vec4 o_color;

vec4 land_at(ivec2 c) {
    return texelFetch(u_land, clamp(c, ivec2(0), ivec2(u_grid) - 1), 0);
}

vec4 cover_at(ivec2 c) {
    return texelFetch(u_cover, clamp(c, ivec2(0), ivec2(u_grid) - 1), 0);
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
    vec3 land = land_color(s, p, coast) * shade;
    float w = aa(coast);
    vec3 c = mix(sea, land, smoothstep(-w, w, coast));
    // Coastline in ink once a cell is a few pixels across.
    float ink = (1.0 - smoothstep(0.0, w * 1.3, abs(coast))) * smoothstep(1.5, 4.0, u_px);
    c = mix(c, vec3(0.20, 0.26, 0.30), ink * 0.85);
    o_color = vec4(c, 1.0);
}
