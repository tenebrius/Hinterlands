// ---- Towns and villages at real scale (R11) ----
// The plan is made on the CPU (town.rs): plots, houses, barns, churches,
// market places, greens, walls. Each shape is a rectangle (or an ellipse) in
// metres; the shader finds the ones listed for its 62.5 m square and draws
// the ground they make (yards, gardens, crofts, cobbles), their shadows, and
// the roofs over them. The streets are drawn as roads between the two.

// Five texels per shape: (centre x, y, half length, half width) in metres,
// then (angle, kind, tone, extra + 4 x height in metres), then for a market
// place its six corners from its centre (R22).
uniform sampler2D u_shapes;
// Shape numbers, four to a texel.
uniform sampler2D u_sidx;

// Most shapes read per square (MAX_SHAPES in relief.rs).
const int SHAPE_MAX = 160;
const int K_PLOT = 0;
const int K_MARKET = 1;
const int K_GREEN = 2;
const int K_CHURCHYARD = 3;
const int K_PRECINCT = 4;
const int K_DITCH = 5;
const int K_COURT = 6;
const int K_HOUSE = 10;
const int K_BARN = 11;
const int K_CHURCH = 12;
const int K_TOWER = 13;
const int K_WALL = 14;
const int K_WALL_TOWER = 15;
const int K_GATE = 16;
const int K_HALL = 17;
const int K_CLOISTER = 18;
const int K_STALL = 20;

struct Town {
    // The ground shape at the point (-1 for none): local position (x along
    // its length; for plots y from the street, 0, to the back, 1, after
    // scaling), half size, tone, extra, coverage.
    int gk;
    vec2 gl;
    vec2 gh;
    float gt;
    float gx;
    float gcov;
    // The building whose roof is seen at the point (-1 for none).
    int bk;
    vec2 bl;
    vec2 bh;
    float bt;
    float bx;
    float bcov;
    float btop;
    float bang;
    // How much of the point lies in a building's shadow.
    float shadow;
    // How far into a built-up area (1 within a few metres of a plot), and
    // the rank of the nearest place (0 hamlet .. 3 city).
    float near;
    float rank;
    // Metres from the point to a market place's edge, negative inside (R22).
    float msd;
    // Metres to the nearest building (R13), so trees keep off the houses.
    float bnear;
};

// The shapes listed for pm's sub-square (R30): `FINE` x `FINE` to a square,
// as in town.rs. A square's entry in the index starts with a (start,
// count) pair per sub-square; `sq` is its texel in the square table.
const int FINE = 4;
void shape_list(vec2 pm, vec4 sq, out int start, out int n) {
    start = 0;
    n = 0;
    if (sq.g < 0.5) {
        return;
    }
    vec2 q = pm / u_cell_m * u_ssub;
    ivec2 f = clamp(ivec2(floor((q - floor(q)) * float(FINE))), ivec2(0), ivec2(FINE - 1));
    int j = int(sq.r) + 2 * (f.y * FINE + f.x);
    start = int(way_texel(u_sidx, j >> 2)[j & 3]);
    j += 1;
    n = min(int(way_texel(u_sidx, j >> 2)[j & 3]), SHAPE_MAX);
}

// Metres from pm to the nearest building, wall or open ground (not plots)
// listed for its square (R27).
float town_sd_at(vec2 pm) {
    int s;
    int e = shape_square(ivec2(floor(pm / u_cell_m * u_ssub)), s);
    if (e < 0) {
        return 1e9;
    }
    vec4 sq = way_texel(u_ssq, e);
    int start;
    int n;
    shape_list(pm, sq, start, n);
    float best = 1e9;
    for (int k = 0; k < SHAPE_MAX; k++) {
        if (k >= n) {
            break;
        }
        int j = start + k;
        int id = int(way_texel(u_sidx, j >> 2)[j & 3]);
        vec4 a = way_texel(u_shapes, 5 * id);
        vec4 b = way_texel(u_shapes, 5 * id + 1);
        int kind = int(b.y + 0.5);
        if (kind == K_PLOT || kind == K_COURT) {
            continue;
        }
        float ca = cos(b.x);
        float sa = sin(b.x);
        vec2 d = pm - a.xy;
        vec2 l = vec2(d.x * ca + d.y * sa, -d.x * sa + d.y * ca);
        vec2 q = abs(l) - a.zw;
        best = min(best, length(max(q, 0.0)) + min(max(q.x, q.y), 0.0));
    }
    return best;
}

Town town_scan(vec2 pm) {
    Town T = Town(-1, vec2(0.0), vec2(1.0), 0.0, 0.0, 0.0,
                  -1, vec2(0.0), vec2(1.0), 0.0, 0.0, 0.0, -1.0, 0.0,
                  0.0, 0.0, 0.0, 1e9, 1e9);
    int s;
    int e = shape_square(ivec2(floor(pm / u_cell_m * u_ssub)), s);
    if (e < 0) {
        return T;
    }
    vec4 sq = way_texel(u_ssq, e);
    int start;
    int n;
    shape_list(pm, sq, start, n);
    float m = mpx();
    float nearest = 1e9;
    for (int k = 0; k < SHAPE_MAX; k++) {
        if (k >= n) {
            break;
        }
        int j = start + k;
        int id = int(way_texel(u_sidx, j >> 2)[j & 3]);
        vec4 a = way_texel(u_shapes, 5 * id);
        vec4 b = way_texel(u_shapes, 5 * id + 1);
        float ca = cos(b.x);
        float sa = sin(b.x);
        vec2 d = pm - a.xy;
        vec2 l = vec2(d.x * ca + d.y * sa, -d.x * sa + d.y * ca);
        int kind = int(b.y + 0.5);
        float height = floor(b.w / 4.0 + 0.01);
        float extra = b.w - 4.0 * height;
        bool round = kind == K_GREEN || kind == K_WALL_TOWER;
        float sd;
        if (kind == K_MARKET) {
            // A market place (R22): a convex six-sided outline.
            vec4 q01 = way_texel(u_shapes, 5 * id + 2);
            vec4 q23 = way_texel(u_shapes, 5 * id + 3);
            vec4 q45 = way_texel(u_shapes, 5 * id + 4);
            vec2 q[6] = vec2[6](q01.xy, q01.zw, q23.xy, q23.zw, q45.xy, q45.zw);
            sd = -1e9;
            for (int e = 0; e < 6; e++) {
                vec2 ea = q[e];
                vec2 eb = q[(e + 1) % 6];
                vec2 t = (eb - ea) / max(length(eb - ea), 1e-3);
                // Corners run anticlockwise (y down), so the outward normal is
                // to the right of each edge.
                sd = max(sd, dot(d - ea, vec2(t.y, -t.x)));
            }
            T.msd = min(T.msd, sd);
        } else if (round) {
            sd = (length(l / a.zw) - 1.0) * min(a.z, a.w);
        } else {
            vec2 q = abs(l) - a.zw;
            sd = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0);
        }
        float cov = clamp(0.5 - sd / m, 0.0, 1.0);
        if (kind < 10) {
            if (kind != K_DITCH && sd < nearest) {
                nearest = sd;
                if (kind == K_PLOT || kind == K_COURT || kind == K_MARKET) {
                    T.rank = extra;
                }
            }
            // Trees keep off the open grounds that are drawn over the land
            // (the ditch, churchyards, greens, the market), not the plots,
            // whose ground is no longer drawn (R27).
            if (kind != K_PLOT && kind != K_COURT) {
                T.bnear = min(T.bnear, sd);
            }
            if (cov > T.gcov) {
                T.gk = kind;
                T.gl = l;
                T.gh = a.zw;
                T.gt = b.z;
                T.gx = extra;
                T.gcov = cov;
            }
        } else {
            // Trees keep off buildings and the wall alike (R27).
            T.bnear = min(T.bnear, sd);
            if (cov > 0.0 && height > T.btop) {
                T.bk = kind;
                T.bl = l;
                T.bh = a.zw;
                T.bt = b.z;
                T.bx = extra;
                T.bcov = cov;
                T.btop = height;
                T.bang = b.x;
            }
            // In shadow if the building stands between the point and the light
            // (the sun 45 degrees up, shadows shortened a little).
            vec2 ds = d + SUN * height * 0.6;
            vec2 ls = vec2(ds.x * ca + ds.y * sa, -ds.x * sa + ds.y * ca);
            float ss;
            if (round) {
                ss = (length(ls / a.zw) - 1.0) * min(a.z, a.w);
            } else {
                vec2 q = abs(ls) - a.zw;
                ss = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0);
            }
            T.shadow = max(T.shadow, clamp(0.5 - ss / m, 0.0, 1.0));
        }
    }
    T.near = 1.0 - smoothstep(2.0, 12.0, nearest);
    return T;
}

// Distance in metres from a point inside a box (local l, half size h) to its edge.
float inner_edge(vec2 l, vec2 h) {
    return min(h.x - abs(l.x), h.y - abs(l.y));
}

// The ground of the town: yards and gardens and crofts within plots, cobbled
// market places, greens, churchyards, precincts, the wall's ditch.
vec3 town_ground(vec3 c, vec2 pm, Town T) {
    // R26: a plot's or court's own ground (yard, garden, fences) is not drawn;
    // the land beneath shows round the buildings.
    if (T.gk < 0 || T.gk == K_PLOT || T.gk == K_COURT) {
        return c;
    }
    // Drawn in the town's own metres (R12): the look's building size scales
    // yards, beds, fences and stalls with the plots.
    float z = u_look.w;
    // How built over the ground round about is, for the town's yards.
    float dense = T.gk == K_PLOT ? smoothstep(0.3, 0.65, built_share(pm)) : 0.0;
    g_unit = z;
    pm /= z;
    vec3 g = c;
    vec2 l = T.gl / z;
    vec2 h = T.gh / z;
    float edge = inner_edge(l, h);
    vec3 grass = turf(grass_color(), pm);
    vec3 earth = vec3(0.55, 0.50, 0.40) * (0.9 + 0.2 * vnoise(pm / 1.5, 701) * soft(1.5));
    if (T.gk == K_PLOT) {
        // Metres from the street front, and the plot's depth.
        float y = l.y + h.y;
        float depth = 2.0 * h.y;
        float town = smoothstep(1.5, 2.5, T.gx);
        float style = T.gt;
        // A trodden yard round the house (about the first 12-16 m), a kitchen
        // garden of beds behind it, then the croft: grass, an orchard, or a
        // crop in strips of its own.
        // In a town's core the yards run most of the way back (R13).
        float yard_end = mix(mix(11.0, 13.0, town) + 3.0 * fract(style * 13.0), depth * 0.85, dense * town);
        vec3 yardc = mix(earth, grass, 0.5 + 0.3 * vnoise(pm / 3.0, 705) * soft(3.0));
        // Garden beds: patches of earth and green, not stripes (R23).
        vec3 beds = mix(vec3(0.46, 0.41, 0.31), vec3(0.40, 0.55, 0.27),
            smoothstep(0.35, 0.65, vnoise(pm / 1.6, 763)) * soft(1.6) + 0.5 * (1.0 - soft(1.6)));
        if (u_field_season == 3) {
            beds = vec3(0.46, 0.41, 0.32);
        }
        if (town < 0.5) {
            // A village lot (R13), after hand-drawn village maps: no fence round
            // it, trodden ground before and about the house, a fenced patch of
            // garden beds, and the rest the village's own grass and trees (the
            // land beneath shows through).
            // The yard: trodden ground round the buildings, its edge ragged.
            float wob = (vnoise(pm / 5.0, 761) - 0.5) * 4.0 + (vnoise(pm / 1.7, 762) - 0.5) * 1.5;
            float yard = 1.0 - smoothstep(-1.0, 1.0, T.bnear / u_look.w - 2.5 - wob);
            vec2 gc = vec2((fract(style * 7.3) - 0.5) * max(h.x - 5.0, 0.0),
                yard_end + 9.0 + fract(style * 3.1) * max(depth - yard_end - 20.0, 0.0));
            vec2 gs = vec2(3.0 + 3.0 * fract(style * 11.0), 2.5 + 2.5 * fract(style * 5.0));
            vec2 gq = abs(vec2(l.x, y) - gc) - gs;
            float gsd = max(gq.x, gq.y);
            float garden = style < 0.75 ? band(gsd, -1e3, 0.0) : 0.0;
            g = mix(c, yardc, yard * 0.5);
            g = mix(g, beds, garden);
            g = mix(g, vec3(0.42, 0.36, 0.28), (style < 0.75 ? band(gsd, -0.25, 0.25) : 0.0) * fine(0.5));
            g_unit = 1.0;
            return mix(c, g, T.gcov);
        }
        // A town lot (R23): no bands across it. Trodden ground round its
        // buildings and along its front, wider where the town is dense; a
        // patch of garden of its own shape away from the buildings; grass
        // (or an orchard) elsewhere.
        float wob = (vnoise(pm / 5.0, 761) - 0.5) * 4.0 + (vnoise(pm / 1.7, 762) - 0.5) * 1.5;
        float bn = T.bnear / u_look.w;
        float yard = max(1.0 - smoothstep(-1.0, 1.0, bn - 3.0 - 6.0 * dense - wob),
                         1.0 - smoothstep(1.0, 2.5, y));
        float gpatch = smoothstep(0.52, 0.6, vnoise(pm / (5.0 + 4.0 * fract(style * 9.0)), 764) + 0.15 * (style - 0.5))
            * smoothstep(2.0, 4.0, bn);
        g = mix(grass, beds, gpatch);
        g = mix(g, yardc, yard);
        // Fruit trees in the orchards, scattered down the plot away from the
        // buildings.
        if (style > 0.3 && style < 0.62 && bn > 5.0 && fine(2.0) > 0.0) {
            vec2 q = vec2(l.x / 7.0, y / 7.5);
            ivec2 b = ivec2(floor(q));
            vec2 o = vec2(0.5) + (vec2(hash(b, 707), hash(b, 708)) - 0.5) * 0.7;
            vec2 dd = (q - vec2(b) - o) * vec2(7.0, 7.5);
            float r = 2.0 + 0.8 * hash(b, 709);
            if (abs(l.x) < h.x - 1.5 && depth - y > 2.0) {
                vec3 leafc = leaf(u_field_season == 3 ? 0 : 1, hash(b, 710)) * (0.75 + 0.35 * clamp(1.0 - length(dd + SUN * r * 0.5) / r, 0.0, 1.0));
                g *= 1.0 - 0.25 * band(length(dd - SUN * 2.0), -r, r);
                g = mix(g, leafc, band(length(dd), -r, r) * fine(2.0));
            }
        }
        // Each plot is fenced or hedged: wattle and walls in towns, hedges in villages.
        // Town lots are divided by a fence or wall along their sides, without
        // which the rows read as one (R13: thin, and not across the front).
        float sides = h.x - abs(l.x);
        g = mix(g, vec3(0.42, 0.36, 0.28), band(min(sides, depth - y), -1.0, 0.3) * fine(0.6));
    } else if (T.gk == K_COURT) {
        // A court: trodden and patched ground, a drain down the alley, a
        // wall round it.
        g = mix(earth, vec3(0.60, 0.57, 0.51), 0.4) * (0.9 + 0.2 * vnoise(pm / 0.9, 751) * soft(0.9));
        g = mix(g, vec3(0.42, 0.38, 0.32), band(edge, -1.0, 0.4) * fine(0.8));
    } else if (T.gk == K_MARKET) {
        // Paved as the streets are, by the roads' own drawing (R22).
        g = c;
    } else if (T.gk == K_GREEN) {
        g = turf(grass_color() * vec3(0.95, 1.06, 0.95), pm);
        // A pond on many greens.
        if (T.gt < 0.45) {
            vec2 pc = vec2(h.x * 0.35, -h.y * 0.2) * (T.gt * 2.0 - 0.45);
            float r = 0.3 * min(h.x, h.y);
            float d = length(l - pc);
            g = mix(g, MUD, band(d, -r - 1.2, r + 1.2));
            g = mix(g, u_field_season == 3 ? vec3(0.78, 0.82, 0.86) : WATER_EDGE, band(d, -r, r));
        }
    } else if (T.gk == K_CHURCHYARD || T.gk == K_PRECINCT) {
        g = turf(grass_color() * vec3(0.92, 1.05, 0.9), pm);
        if (T.gk == K_CHURCHYARD) {
            // Gravestones in rough rows.
            vec2 q = vec2(l.x / 2.2, l.y / 3.0);
            ivec2 b = ivec2(floor(q));
            vec2 o = vec2(0.3 + 0.4 * hash(b, 711), 0.5);
            float stone = band(length((q - vec2(b) - o) * vec2(2.2, 3.0) * vec2(1.0, 2.0)), -0.5, 0.5);
            g = mix(g, vec3(0.62, 0.61, 0.58), stone * step(0.35, hash(b, 713)) * fine(0.6));
        }
        // The boundary wall.
        g = mix(g, vec3(0.58, 0.55, 0.50), band(edge, -1.0, 0.8));
    } else if (T.gk == K_DITCH) {
        float wet = 1.0 - smoothstep(0.0, 0.3, T.gt);
        vec3 water = u_field_season == 3 ? vec3(0.75, 0.80, 0.84) : WATER_EDGE * 0.95;
        // Shaded across its width, not from each piece's ends, so the pieces
        // join round the wall's curves (R14).
        float mid = 1.0 - abs(l.y) / h.y;
        g = mix(grass * 0.75, water, smoothstep(0.3, 0.8, mid) * (0.5 + 0.5 * wet));
    }
    g_unit = 1.0;
    return mix(c, g, T.gcov);
}

// A pitched roof seen from above: local position l, half size h, the ridge
// along the longer side; `hip` gives hipped ends. Brightness of the slope.
float roof_light(vec2 l, vec2 h, float ang, bool hip) {
    bool along_x = h.x >= h.y;
    vec2 n;
    if (along_x) {
        n = vec2(0.0, sign(l.y));
        if (hip && abs(l.x) - (h.x - h.y) > abs(l.y)) {
            n = vec2(sign(l.x), 0.0);
        }
    } else {
        n = vec2(sign(l.x), 0.0);
        if (hip && abs(l.y) - (h.y - h.x) > abs(l.x)) {
            n = vec2(0.0, sign(l.y));
        }
    }
    float ca = cos(ang);
    float sa = sin(ang);
    vec2 w = vec2(n.x * ca - n.y * sa, n.x * sa + n.y * ca);
    return 0.88 + 0.22 * dot(w, SUN);
}

vec3 roof_color(float material, float tone, vec2 l, vec2 h) {
    vec3 c;
    if (material < 0.5) {
        // Thatch, golden when new and grey with age.
        c = mix(vec3(0.74, 0.62, 0.38), vec3(0.56, 0.51, 0.42), tone);
        bool along_x = h.x >= h.y;
        float across = along_x ? l.y : l.x;
        float alongr = along_x ? l.x : l.y;
        c *= 0.92 + 0.14 * vnoise(vec2(alongr / 0.35, across / 2.0), 721) * fine(0.35);
    } else if (material < 1.5) {
        // Tiles, red to brown, in courses along the ridge.
        c = mix(vec3(0.62, 0.30, 0.21), vec3(0.52, 0.36, 0.27), tone);
        bool along_x = h.x >= h.y;
        float across = along_x ? l.y : l.x;
        c *= 1.0 - 0.08 * step(0.5, fract(across / 0.3)) * fine(0.3);
        c *= 0.95 + 0.1 * vnoise(l / 1.5, 723) * soft(1.5);
    } else {
        // Stone slates.
        c = mix(vec3(0.42, 0.42, 0.43), vec3(0.50, 0.48, 0.45), tone);
        c *= 0.94 + 0.12 * vnoise(l / 0.8, 725) * soft(0.8);
    }
    return c;
}

const vec3 STONE_TOP = vec3(0.68, 0.65, 0.58);
const vec3 LEAD = vec3(0.50, 0.52, 0.54);

// The roofs, towers and walls.
vec3 town_buildings(vec3 c, Town T) {
    if (T.bk < 0) {
        return c;
    }
    float z = u_look.w;
    vec2 l = T.bl / z;
    vec2 h = T.bh / z;
    // Round towers measure their edge round, for the eaves and the ink.
    float edge = T.bk == K_WALL_TOWER ? (1.0 - length(l / h)) * min(h.x, h.y) : inner_edge(l, h);
    vec3 r;
    g_unit = z;
    if (T.bk == K_HOUSE || T.bk == K_BARN || T.bk == K_HALL) {
        r = roof_color(T.bx, T.bt, l, h) * (T.bk == K_BARN ? 0.9 : 1.0);
        r *= roof_light(l, h, T.bang, T.bx < 0.5);
        // The ridge, and a chimney on most houses.
        float ridge = h.x >= h.y ? abs(l.y) : abs(l.x);
        r = mix(r, r * 0.8, band(ridge, -0.25, 0.25) * fine(0.5));
        if (T.bk == K_HOUSE && T.bt > 0.3) {
            vec2 ch = h.x >= h.y ? vec2(h.x - 1.4, 0.0) : vec2(0.0, h.y - 1.4);
            ch *= T.bt > 0.65 ? -1.0 : 1.0;
            vec2 q = abs(l - ch);
            r = mix(r, vec3(0.32, 0.27, 0.24), band(max(q.x, q.y), -1.0, 0.5) * fine(1.0));
        }
    } else if (T.bk == K_CHURCH) {
        // A chancel narrower than the nave at the east end.
        if (l.x > h.x * 0.45 && abs(l.y) > h.y * 0.78) {
            g_unit = 1.0;
            return c;
        }
        r = (T.bx > 0.5 ? LEAD : roof_color(2.0, T.bt, l, h)) * roof_light(l, h, T.bang, false);
    } else if (T.bk == K_TOWER || T.bk == K_GATE) {
        // A low pyramid roof behind a parapet.
        vec2 n = abs(l.x) > abs(l.y) ? vec2(sign(l.x), 0.0) : vec2(0.0, sign(l.y));
        float ca = cos(T.bang);
        float sa = sin(T.bang);
        vec2 w = vec2(n.x * ca - n.y * sa, n.x * sa + n.y * ca);
        r = LEAD * (0.8 + 0.3 * dot(w, SUN));
        r = mix(r, STONE_TOP, band(edge, -1.0, 1.0));
    } else if (T.bk == K_CLOISTER) {
        // Ranges round a garth of grass.
        float wide = 5.0;
        if (edge > wide) {
            r = turf(grass_color(), l);
        } else {
            vec2 n = abs(l.x) / h.x > abs(l.y) / h.y ? vec2(sign(l.x), 0.0) : vec2(0.0, sign(l.y));
            float outer = edge < wide * 0.5 ? 1.0 : -1.0;
            float ca = cos(T.bang);
            float sa = sin(T.bang);
            vec2 w = vec2(n.x * ca - n.y * sa, n.x * sa + n.y * ca) * outer;
            r = LEAD * (0.82 + 0.3 * dot(w, SUN));
        }
    } else if (T.bk == K_STALL) {
        // A market stall (R25): an awning striped across in two colours,
        // lit on its slope.
        vec3 a1 = T.bt < 0.33 ? vec3(0.70, 0.22, 0.18) : T.bt < 0.66 ? vec3(0.78, 0.62, 0.25) : vec3(0.25, 0.42, 0.55);
        vec3 a2 = vec3(0.90, 0.86, 0.76);
        r = mix(a1, a2, step(0.5, fract(l.x / 0.7)) * fine(0.35));
        r *= roof_light(l, h, T.bang, false);
    } else if (T.bk == K_WALL) {
        // The curtain wall (R25): a broad band of dressed stone, the wall walk
        // down its middle between two continuous parapets, battlements only on
        // the outer one (`extra` 1 when the outside is to the left of its
        // run), and its faces dark.
        float side = h.y - abs(l.y);
        // Pieces join end to end: only the long faces are edges.
        edge = side;
        bool outer = (l.y > 0.0) == (T.bx > 0.5);
        vec3 stone = vec3(0.63, 0.60, 0.54) * (0.94 + 0.1 * vnoise(l / 1.2, 731) * soft(1.2));
        r = stone * 0.9;
        float pw = 0.25 * h.y + 0.3;
        r = mix(r, stone * 1.06, band(side, 0.0, pw));
        // Battlements: small merlons along the outer parapet only, faint so
        // they do not read as sleepers.
        float crenel = step(0.6, fract(l.x / 1.8)) * (outer ? 1.0 : 0.0);
        r = mix(r, stone * 0.8, band(side, 0.1, pw - 0.2) * crenel * fine(0.6));
        r = mix(r, vec3(0.20, 0.18, 0.16), band(side, -1.0, max(0.18, 0.7 * mpx())));
    } else if (T.bk == K_WALL_TOWER) {
        // A round tower with a conical roof.
        vec2 n = normalize(l + 1e-4);
        float ca = cos(T.bang);
        float sa = sin(T.bang);
        vec2 w = vec2(n.x * ca - n.y * sa, n.x * sa + n.y * ca);
        r = roof_color(2.0, T.bt, l, h) * (0.8 + 0.3 * dot(w, SUN));
    } else {
        g_unit = 1.0;
        return c;
    }
    // Eaves: a dark line where the roof meets its shadow side.
    r = mix(r, r * 0.7, band(edge, -1.0, 0.25) * fine(0.5));
    // The outline in ink (R14).
    r = mix(r, INK, band(edge, -1.0, max(0.25, 1.3 * mpx())) * u_ink);
    if (u_season == 3) {
        r = mix(r, vec3(0.93, 0.94, 0.96), 0.55);
    }
    g_unit = 1.0;
    return mix(c, r, T.bcov);
}
