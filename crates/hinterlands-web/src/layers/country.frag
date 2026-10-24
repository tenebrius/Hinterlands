// ---- The countryside at real scale (R10) ----
// Fields, woods and pasture as they would look from the air at a metre a pixel,
// each thing fading in once it is a few pixels across. The detail is invented,
// but where it lies is not: which land is farmed, wooded or grazed, and since
// when, comes from the cells. Everything is seeded from things that persist
// (map position, the cells' history), so it stays put from year to year.

// Per cell: r wetness (0 dry .. 1 on a river's flood plain), g the year first
// farmed in this session / 100 (-1 if never), b the original forest cover.
uniform sampler2D u_ground;

// Value noise on two turned grids, so the lattice of either does not show as
// blocks aligned with the map.
float mottle(vec2 p, int k) {
    vec2 a = mat2(0.8253, 0.5646, -0.5646, 0.8253) * p;
    vec2 b = mat2(-0.3233, 0.9463, -0.9463, -0.3233) * p * 1.618;
    return 0.6 * vnoise(a, k) + 0.4 * vnoise(b + 17.3, k + 1);
}

// Towns (R11): per map cell, where its block of index squares starts in
// u_ssq, or -1; per square, its first entry in u_sidx, how many, the share of
// it built over, and 1 if it is town land; squares per cell along a side.
uniform sampler2D u_scell;
uniform sampler2D u_ssq;
uniform float u_ssub;

// Texel k of a data texture laid out in rows of 2048.
vec4 row_texel(sampler2D s, int k) {
    return texelFetch(s, ivec2(k % 2048, k / 2048), 0);
}

int shape_square(ivec2 sq, out int start) {
    int sub = int(u_ssub + 0.5);
    ivec2 cell = ivec2(floor(vec2(sq) / float(sub)));
    start = 0;
    if (cell.x < 0 || cell.y < 0 || cell.x >= int(u_grid.x) || cell.y >= int(u_grid.y)) {
        return -1;
    }
    float first = texelFetch(u_scell, cell, 0).r;
    if (first < 0.0) {
        return -1;
    }
    ivec2 loc = sq - cell * sub;
    return int(first) + loc.y * sub + loc.x;
}

// A square's value near pm, smoothly between squares: channel 2 the share
// built over, channel 3 town land.
float square_smooth(vec2 pm, int ch) {
    vec2 q = pm / u_cell_m * u_ssub - 0.5;
    ivec2 b = ivec2(floor(q));
    vec2 f = smoothstep(0.0, 1.0, q - vec2(b));
    float v[4];
    for (int k = 0; k < 4; k++) {
        int s;
        int e = shape_square(b + ivec2(k & 1, k >> 1), s);
        v[k] = e < 0 ? 0.0 : row_texel(u_ssq, e)[ch];
    }
    return mix(mix(v[0], v[1], f.x), mix(v[2], v[3], f.x), f.y);
}

// The share of the ground built over near pm.
float built_share(vec2 pm) {
    return square_smooth(pm, 2);
}

// Town land near pm: above 0 inside a place's edge, below outside, roughly in
// metres (the field falls by 1 over 2.6 squares). The edge lies some 80 m out
// from the last plots, wandering at two scales.
float town_land(vec2 pm) {
    float v = square_smooth(pm, 3) - 0.5
        + (mottle(pm / 140.0, 741) - 0.5) * 0.3 + (mottle(pm / 35.0, 742) - 0.5) * 0.12;
    return v * 162.0;
}

// The density of a place's groves and single trees at pm (R13, R27): clumps
// of about 70 m (times the tree scale) among its closes and lots and round
// its edge, none where the ground is densely built over.
float grove_at(vec2 pm) {
    float inside = smoothstep(-25.0, 5.0, town_land(pm));
    if (inside <= 0.0) {
        return 0.0;
    }
    float clump = smoothstep(0.5, 0.72, mottle(pm / (70.0 * u_look.z), 771));
    return (0.07 + 0.5 * clump) * inside * (1.0 - smoothstep(0.3, 0.55, built_share(pm)));
}

// Towards the light (north-west, 45 degrees up), in map space with y down.
const vec2 SUN = vec2(-0.7071, -0.7071);
const vec3 SUN3 = vec3(-0.5, -0.5, 0.7071);

// Metres per physical pixel.
float mpx() {
    return u_cell_m / u_px / g_unit;
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

// The same for mottling (noise rather than distinct things), which reads as
// grain until it is several pixels across.
float soft(float metres) {
    return smoothstep(4.0, 10.0, metres / mpx());
}

// A hash of a parcel or tree and a number within it.
float hash2(ivec2 id, int k, int salt) {
    return hash(id * 7919 + ivec2(k * 104729, k * 31), salt);
}

vec4 ground_at(ivec2 c) {
    return texelFetch(u_ground, clamp(c, ivec2(0), ivec2(u_grid) - 1), 0);
}

vec4 ground_smooth(vec2 p) {
    vec2 q = p - 0.5;
    vec2 b = floor(q);
    vec2 f = smoothstep(0.0, 1.0, q - b);
    ivec2 ib = ivec2(b);
    return mix(
        mix(ground_at(ib), ground_at(ib + ivec2(1, 0)), f.x),
        mix(ground_at(ib + ivec2(0, 1)), ground_at(ib + ivec2(1, 1)), f.x),
        f.y);
}

// Rise per metre round the cell holding map point p (cells).
vec2 grad_at(vec2 p) {
    ivec2 c = ivec2(floor(p));
    float ex = land_at(c + ivec2(1, 0)).r - land_at(c - ivec2(1, 0)).r;
    float ey = land_at(c + ivec2(0, 1)).r - land_at(c - ivec2(0, 1)).r;
    return vec2(ex, ey) / (2.0 * u_cell_m);
}

// ---- Parcels ----
// The land is cut into parcels of about 170 m: a Voronoi tessellation of sites
// scattered irregularly (one to a square, anywhere in it), with the edges bent
// a little so no boundary is straight. A farmed parcel is a furlong of strips,
// a meadow, a close, common grazing or scrub.

const float PARCEL = 170.0;

vec2 parcel_off(ivec2 c, int salt) {
    return 0.1 + 0.8 * vec2(hash(c, salt), hash(c, salt + 2));
}

struct Parcels {
    // This parcel and the one across the nearest boundary.
    ivec2 a;
    ivec2 b;
    // Their sites, metres.
    vec2 sa;
    vec2 sb;
    // Metres to that boundary.
    float edge;
    // Metres along it from the sites' midpoint, the same seen from either side.
    float along;
    // Unit normal of the boundary, from this parcel towards the other.
    vec2 n;
};

// Parcels of about `size` metres (sites seeded by `salt`).
Parcels parcels(vec2 pm, float size, int salt) {
    // Boundaries bend some 30 m over a quarter of a kilometre, and a few metres
    // over a strip's width, so none runs straight.
    // Over most of a kilometre the parcels stretch and squeeze, so their
    // spacing varies and no lattice shows through.
    pm += (vec2(vnoise(pm / 900.0, 435), vnoise(pm / 900.0, 437)) - 0.5) * 2.4 * size
        + (vec2(vnoise(pm / 260.0, 405), vnoise(pm / 260.0, 407)) - 0.5) * 60.0
        + (vec2(vnoise(pm / 60.0, 431), vnoise(pm / 60.0, 433)) - 0.5) * 14.0;
    vec2 q = pm / size;
    ivec2 base = ivec2(floor(q));
    vec2 f = q - vec2(base);
    Parcels P;
    float best = 1e9;
    vec2 ra = vec2(0.0);
    ivec2 ga = ivec2(0);
    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            ivec2 g = ivec2(dx, dy);
            vec2 r = vec2(g) + parcel_off(base + g, salt) - f;
            float d = dot(r, r);
            if (d < best) {
                best = d;
                ra = r;
                ga = g;
            }
        }
    }
    best = 1e9;
    vec2 rb = ra + vec2(1.0, 0.0);
    ivec2 gb = ga;
    for (int dy = -2; dy <= 2; dy++) {
        for (int dx = -2; dx <= 2; dx++) {
            ivec2 g = ga + ivec2(dx, dy);
            if (g == ga) {
                continue;
            }
            vec2 r = vec2(g) + parcel_off(base + g, salt) - f;
            float d = dot(0.5 * (ra + r), normalize(r - ra));
            if (d < best) {
                best = d;
                rb = r;
                gb = g;
            }
        }
    }
    P.a = base + ga;
    P.b = base + gb;
    P.sa = pm + ra * size;
    P.sb = pm + rb * size;
    P.edge = best * size;
    P.n = normalize(rb - ra);
    bool first = P.a.x < P.b.x || (P.a.x == P.b.x && P.a.y < P.b.y);
    vec2 t = vec2(-P.n.y, P.n.x) * (first ? 1.0 : -1.0);
    P.along = dot(-0.5 * (ra + rb), t) * size;
    return P;
}

const int OPEN = 0;
const int FURLONG = 1;
const int MEADOW = 2;
const int COMMON = 3;
const int CLOSE_GRASS = 4;
const int CLOSE_CROP = 5;
const int SCRUB = 6;
// Inside a place's boundary: its own closes, gardens, orchards and paddocks.
const int TOWN_CLOSE = 7;

struct Plot {
    int use;
    // The holding (owner hash, -1 for none).
    float owner;
    // Years farmed; hedged once farmed fifty (R3's rule).
    float age;
    bool hedged;
    // Direction the strips run, radians.
    float angle;
    // Which of the township's three great fields (for the rotation).
    int field;
    // 0 light .. 1 heavy land (higher ridges).
    float heavy;
    float wet;
    // Fade of a field gone out of cultivation (R3).
    float ghost;
    float h;
};

// Which of three great fields a furlong lies in: a coarser tessellation, so
// neighbouring furlongs share a crop.
int great_field(vec2 sm) {
    vec2 q = sm / 650.0;
    ivec2 b = ivec2(floor(q));
    vec2 f = q - vec2(b);
    float best = 9.0;
    ivec2 id = b;
    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            ivec2 g = ivec2(dx, dy);
            vec2 r = vec2(g) + 0.15 + 0.7 * vec2(hash(b + g, 421), hash(b + g, 422)) - f;
            float d = dot(r, r);
            if (d < best) {
                best = d;
                id = b + g;
            }
        }
    }
    return int(hash(id, 423) * 3.0) % 3;
}

// What a parcel is, from the cells at its site (sm, metres).
Plot plot_of(ivec2 id, vec2 sm) {
    vec2 sc = sm / u_cell_m;
    vec4 cv = cover_smooth(sc);
    // The same outline as the fields drawn far out, met parcel by parcel.
    float fm = cv.g + (fbm(sc * 2.2, 71) - 0.5) * 0.45;
    vec4 pl = plot_at(ivec2(floor(sc)));
    vec4 gr = ground_smooth(sc);
    vec4 ld = land_at(ivec2(floor(sc)));
    vec2 g = grad_at(sc);
    Plot P;
    P.h = hash(id, 409);
    P.owner = pl.r;
    P.ghost = cv.a;
    P.wet = gr.r;
    float first = gr.g * 100.0;
    P.age = gr.g >= 0.0 ? max(u_year - first, 0.0) : 0.0;
    P.hedged = P.age >= 50.0;
    float steep = length(g);
    P.heavy = smoothstep(0.3, 0.8, ld.b) * (1.0 - smoothstep(0.02, 0.08, steep));
    P.use = OPEN;
    float u = hash(id, 411);
    if (fm > 0.5) {
        // Land cleared late from woodland (assarts) lies in small hedged closes;
        // the old land is open field.
        float assart = smoothstep(0.3, 0.55, gr.b) * smoothstep(1.2, 2.5, gr.g);
        if (u < 0.6) {
            P.use = hash(id, 413) < assart ? CLOSE_CROP : FURLONG;
        } else {
            float r = (u - 0.6) / 0.4;
            if (r < P.wet * 0.9) {
                P.use = MEADOW;
            } else if (r < 0.55) {
                P.use = COMMON;
            } else if (r < 0.85) {
                P.use = hash(id, 413) < assart + 0.3 ? CLOSE_GRASS : COMMON;
            } else {
                P.use = SCRUB;
            }
        }
    }
    // Strips run away from the nearest road or river (R3), each furlong at its
    // own angle; on wet or heavy sloping land they run downhill, to drain.
    bool horiz = mod(pl.g, 2.0) > 0.5;
    P.angle = (horiz ? 0.0 : 1.5708) + (hash(id, 415) - 0.5) * 1.7;
    if (steep > 0.012 && (P.wet > 0.2 || P.heavy > 0.5)) {
        P.angle = atan(g.y, g.x) + (hash(id, 415) - 0.5) * 0.3;
    }
    P.field = great_field(sm);
    return P;
}

// The crop a great field carries this year: 0 winter corn, 1 spring corn, 2 fallow.
int crop_of(int field) {
    int k = (field + int(u_year)) % 3;
    return k == 0 ? 2 : k - 1;
}

vec3 crop_color(int crop) {
    return FIELDS[u_field_season * 3 + crop];
}

// The mean colour of a crop as seen from afar.
vec3 grass_color() {
    vec3 g = vec3(0.52, 0.63, 0.34);
    if (u_field_season == 0) {
        g = vec3(0.50, 0.66, 0.32);
    } else if (u_field_season == 2) {
        g = mix(g, vec3(0.60, 0.57, 0.36), 0.4);
    } else if (u_field_season == 3) {
        g = vec3(0.58, 0.59, 0.47);
    }
    return g;
}

// Turf: the grass colour mottled at three scales.
vec3 turf(vec3 g, vec2 pm) {
    g *= 0.89 + 0.22 * mottle(pm / 60.0, 441) * soft(60.0);
    g *= 0.90 + 0.20 * mottle(pm / 9.0, 443) * soft(9.0);
    g *= 0.92 + 0.16 * mottle(pm / 1.6, 445) * soft(1.6);
    return g;
}

// Ridges: a ridge of height h (0..1) across a band of width w at t (0..1),
// lit from the sun: brightness factor.
float ridge_light(float t, float h, vec2 nrm) {
    return 1.0 - 0.11 * h * cos(3.14159 * t) * dot(nrm, SUN);
}

// A furlong: strips of 10-20 m, each a ridge, with the reversed S of the plough
// team, a crop by the rotation and a tone of its own.
vec3 furlong(Plot P, ivec2 id, vec2 pm, vec2 site, float strips) {
    vec2 dir = vec2(cos(P.angle), sin(P.angle));
    vec2 nrm = vec2(-dir.y, dir.x);
    vec2 rel = pm - site;
    float along = dot(rel, dir);
    float across = dot(rel, nrm) + 3.5 * sin(along * 0.0285 + P.h * 6.2832);
    float w0 = mix(11.0, 19.0, hash(id, 417));
    float s = across / w0;
    float k = floor(s);
    float lo = k + 0.3 * (hash2(id, int(k), 427) - 0.5);
    float hi;
    if (s < lo) {
        hi = lo;
        k -= 1.0;
        lo = k + 0.3 * (hash2(id, int(k), 427) - 0.5);
    } else {
        hi = k + 1.0 + 0.3 * (hash2(id, int(k) + 1, 427) - 0.5);
        if (s >= hi) {
            k += 1.0;
            lo = hi;
            hi = k + 1.0 + 0.3 * (hash2(id, int(k) + 1, 427) - 0.5);
        }
    }
    float t = (s - lo) / (hi - lo);
    float sw = (hi - lo) * w0;
    int crop = crop_of(P.field);
    vec3 mean = crop_color(crop);
    float hs = hash2(id, int(k), 429);
    vec3 col = mean * (0.92 + 0.16 * hs);
    // In summer some strips of winter corn are already reaped.
    if (u_field_season == 1 && crop == 0 && hs < 0.3) {
        col = mix(col, vec3(0.86, 0.80, 0.58), 0.7);
    }
    // Rows along the strip.
    col *= 1.0 + 0.08 * (vnoise(vec2(along / 3.0, across / 0.6), 431) - 0.5) * fine(0.6);
    // Ridge and furrow: higher on heavy land; the furrows between ridges darker.
    float h = mix(0.45, 1.0, P.heavy);
    col *= mix(1.0, ridge_light(t, h, nrm), fine(sw * 0.5));
    col *= 1.0 - 0.09 * band(min(t, 1.0 - t) * sw, -0.4, 0.4);
    return mix(mean, col, strips);
}

// Crown of a tree or bush. `top` is the height of its top surface at the point,
// `cover` how much of the pixel it covers.
struct Crown {
    vec3 col;
    float cover;
    float top;
    int sp;
};

// Species: 0 oak, 1 beech, 2 alder, 3 willow, 4 birch, 5 hawthorn and gorse,
// 6 hazel underwood.
vec3 leaf(int sp, float v) {
    vec3 c;
    int s = u_field_season;
    if (sp == 0) {
        c = s == 2 ? vec3(0.52, 0.44, 0.20) : vec3(0.29, 0.40, 0.19);
    } else if (sp == 1) {
        c = s == 2 ? vec3(0.70, 0.40, 0.15) : vec3(0.32, 0.48, 0.20);
    } else if (sp == 2) {
        c = s == 2 ? vec3(0.30, 0.37, 0.20) : vec3(0.20, 0.33, 0.20);
    } else if (sp == 3) {
        c = s == 2 ? vec3(0.56, 0.56, 0.32) : vec3(0.44, 0.53, 0.36);
    } else if (sp == 4) {
        c = s == 2 ? vec3(0.78, 0.63, 0.22) : vec3(0.40, 0.53, 0.26);
    } else if (sp == 5) {
        c = vec3(0.27, 0.34, 0.18);
        if (s == 0) {
            c = mix(c, vec3(0.80, 0.72, 0.25), 0.35 * step(0.5, v));
        }
    } else {
        c = s == 2 ? vec3(0.55, 0.50, 0.24) : vec3(0.33, 0.45, 0.22);
    }
    if (s == 0 && sp != 5) {
        c = mix(c, vec3(0.50, 0.66, 0.30), 0.35);
    }
    // Winter: bare twigs over the ground, except gorse.
    if (s == 3 && sp != 5) {
        c = mix(vec3(0.40, 0.35, 0.31), vec3(0.47, 0.42, 0.36), v);
    }
    return c * (0.9 + 0.2 * v);
}

// The species of a tree from the ground: alder and willow by wet streams, beech
// on dry upland, birch on poor land, oak elsewhere. `pick` 0..1.
int species(vec4 env, float pick) {
    float wet = env.x;
    float up = env.y;
    float poor = env.z;
    float a = 0.55 * wet;
    float w = a + 0.3 * wet;
    float b = w + (1.0 - wet) * 0.55 * up;
    float bi = b + (1.0 - wet) * 0.35 * poor;
    return pick < a ? 2 : pick < w ? 3 : pick < b ? 1 : pick < bi ? 4 : 0;
}

// Trees are scattered from three jittered grids at different angles and sizes,
// each with a third of the trees, so no lattice shows through.
const mat2 GRID_ROT[3] = mat2[3](
    mat2(1.0, 0.0, 0.0, 1.0),
    mat2(0.7446, 0.6675, -0.6675, 0.7446),
    mat2(-0.1388, 0.9903, -0.9903, -0.1388));
const float GRID_SIZE[3] = float[3](1.0, 1.13, 0.91);

// Centre (metres) of candidate tree c of grid g, and its salt.
vec2 tree_centre(int g, ivec2 c, float cs, int sg) {
    return transpose(GRID_ROT[g]) * ((vec2(c) + 0.05 + 0.9 * vec2(hash(c, sg), hash(c, sg + 1))) * cs);
}

// The largest crown radius from rmin..rmax (lone trees grow by up to a
// quarter), with a little slack for rounding (R29).
float crown_reach(float rmin, float rmax) {
    return 1.25 * max(rmin, rmax) * 1.001 + 1e-3;
}

// The candidate cells of grid g, within the 3 x 3 about b, that may hold a
// tree centre within `reach` metres of pm (R29): a centre lies within its
// cell's middle nine-tenths, so a cell is passed over when that band lies
// further than `reach` along either of the grid's axes. The rotations are
// not exactly of unit scale (s, their squared scale, is off by up to 4e-5, a
// metre and more this far from the origin), so the point is taken into the
// frame `tree_centre` maps back from, and distances there scaled by the
// square root of s, with slack for rounding. Returns the first and last
// cells along each axis.
ivec4 cell_range(int g, vec2 pm, ivec2 b, float cs, float reach) {
    float s = dot(GRID_ROT[g][0], GRID_ROT[g][0]);
    vec2 q = GRID_ROT[g] * pm / (s * cs);
    float rr = (reach + 0.01 * cs) / (cs * sqrt(s));
    ivec2 lo = max(b - 1, ivec2(ceil(q - rr - 0.95)));
    ivec2 hi = min(b + 1, ivec2(floor(q + rr - 0.05)));
    return ivec4(lo, hi);
}

// Trees about `cellm` metres apart. A tree stands where `dens` (the density at
// the point, with its gradient per metre `dgrad`, so a crown is kept or not as
// a whole) beats its own number; radius rmin..rmax metres. env: wetness,
// upland, poor land, and 1 for bushes, 2 for hazel underwood. The tallest
// crown at the point is the one seen.
Crown crowns_at(vec2 pm, float cellm, float dens, vec2 dgrad, float rmin, float rmax, int salt, vec4 env) {
    Crown best = Crown(vec3(0.0), 0.0, -1.0, -1);
    float m = mpx();
    // The search keeps only the tallest crown's geometry; its colour is
    // worked out once, after it (R29).
    ivec2 bc = ivec2(0);
    int bsg = 0;
    vec2 bctr = vec2(0.0);
    vec2 bd = vec2(0.0);
    float br = 0.0;
    float bdd = 0.0;
    float bz = 0.0;
    // No crown reaches the pixel from further than this (R29): candidates
    // beyond it are passed over before their density is looked up.
    float reach = crown_reach(rmin, rmax) + 0.5 * m;
    for (int g = 0; g < 3; g++) {
        float cs = cellm * GRID_SIZE[g] * 1.7320508;
        int sg = salt + 40 * g;
        ivec2 b = ivec2(floor(GRID_ROT[g] * pm / cs));
        ivec4 cr = cell_range(g, pm, b, cs, reach);
        for (int y = cr.y; y <= cr.w; y++) {
            for (int x = cr.x; x <= cr.z; x++) {
                ivec2 c = ivec2(x, y);
                vec2 ctr = tree_centre(g, c, cs, sg);
                vec2 d = pm - ctr;
                if (length(d) > reach) {
                    continue;
                }
                float keep = clamp(dens + dot(dgrad, ctr - pm), 0.0, 1.0);
                if (g_grove_on) {
                    keep = max(keep, 0.9 * grove_at(ctr * g_unit));
                }
                if (hash(c, sg + 2) >= keep) {
                    continue;
                }
                // Trees standing alone (where few are kept) grow broad.
                float r = mix(rmin, rmax, hash(c, sg + 3)) * mix(1.25, 1.0, smoothstep(0.1, 0.5, keep));
                float dd = length(d);
                // Only a crown that covers the pixel competes to be seen
                // (R27): one just beyond it hid its neighbours' crowns.
                if (dd > r + 0.5 * m) {
                    continue;
                }
                float z = sqrt(max(r * r - dd * dd, 0.0));
                float top = z + r * 0.9 + 2.0 * hash(c, sg + 4);
                // Clearance is looked up only for a crown that would be seen
                // (R29); the order of the two tests does not change which.
                if (top <= best.top || !tree_clear(ctr * g_unit, r * g_unit)) {
                    continue;
                }
                best.top = top;
                bc = c;
                bsg = sg;
                bctr = ctr;
                bd = d;
                br = r;
                bdd = dd;
                bz = z;
            }
        }
    }
    if (best.top < 0.0) {
        return best;
    }
    // Trees of a kind grow in stands: the pick drifts over some 150 m.
    float pick = fract(hash(bc, bsg + 5) * 0.6 + vnoise(bctr / 150.0, salt + 6));
    int sp = env.w > 1.5 ? 6 : env.w > 0.5 ? 5 : species(env, pick);
    float v = hash(bc, bsg + 7);
    vec3 n = normalize(vec3(bd, max(bz, 0.2 * br)));
    float lit = clamp(dot(n, SUN3), 0.0, 1.0);
    vec3 col = leaf(sp, v) * (0.55 + 0.65 * lit);
    // Clumps of leaves.
    col *= 0.86 + 0.28 * vnoise((pm + vec2(bc) * 13.7) / 1.3, salt + 8) * fine(1.3);
    float cover = clamp((br - bdd) / m + 0.5, 0.0, 1.0);
    // Its outline in ink (R14).
    col = mix(col, col * 0.35, (1.0 - smoothstep(0.0, max(0.3, 1.3 * m), br - bdd)) * u_ink);
    if (u_field_season == 3 && sp != 5) {
        cover *= 0.75;
    }
    return Crown(col, cover, best.top, sp);
}

// The highest crown top near a point, for shadows.
// `here` is the pixel where `dens` and `dgrad` were taken (R27): a tree's
// density is extrapolated from there, so a crown and its shadow agree on
// which trees stand.
// The search stops once the top is past `full` (R29): from there the shadow
// it casts is at full strength, whatever stands higher.
float crown_top(vec2 pm, vec2 here, float cellm, float dens, vec2 dgrad, float rmin, float rmax, int salt,
                float full) {
    float best = -1.0;
    float reach = crown_reach(rmin, rmax);
    for (int g = 0; g < 3; g++) {
        float cs = cellm * GRID_SIZE[g] * 1.7320508;
        int sg = salt + 40 * g;
        ivec2 b = ivec2(floor(GRID_ROT[g] * pm / cs));
        ivec4 cr = cell_range(g, pm, b, cs, reach);
        for (int y = cr.y; y <= cr.w; y++) {
            for (int x = cr.x; x <= cr.z; x++) {
                ivec2 c = ivec2(x, y);
                vec2 ctr = tree_centre(g, c, cs, sg);
                if (length(pm - ctr) > reach) {
                    continue;
                }
                float keep = clamp(dens + dot(dgrad, ctr - here), 0.0, 1.0);
                if (g_grove_on) {
                    keep = max(keep, 0.9 * grove_at(ctr * g_unit));
                }
                if (hash(c, sg + 2) >= keep) {
                    continue;
                }
                float r = mix(rmin, rmax, hash(c, sg + 3)) * mix(1.25, 1.0, smoothstep(0.1, 0.5, keep));
                float dd = length(pm - ctr);
                if (dd < r) {
                    float t = sqrt(r * r - dd * dd) + r * 0.9 + 2.0 * hash(c, sg + 4);
                    // Clearance only for a crown that would raise the top (R29).
                    if (t > best && tree_clear(ctr * g_unit, r * g_unit)) {
                        best = t;
                        if (best >= full) {
                            return best;
                        }
                    }
                }
            }
        }
    }
    return best;
}

// Draws crowns over c, with their shadows cast away from the light. `under`
// is the colour between crowns (the woodland floor), used where `floor` is 1.
vec3 draw_crowns(vec3 c, vec2 pm, float cellm, float dens, vec2 dgrad, float rmin, float rmax,
                 int salt, vec4 env, vec3 under, float floorc) {
    Crown cr = crowns_at(pm, cellm, dens, dgrad, rmin, rmax, salt, env);
    float own = max(cr.top, 0.0) * cr.cover;
    float sh = 0.4 * (rmin + rmax);
    // A top this far above the pixel's own casts full shadow (the smoothstep
    // below reaches 1 at 1.5; a little more for rounding).
    float full = own + sh * 0.7 + 1.501;
    float lit = crown_top(pm + SUN * sh, pm, cellm, dens, dgrad, rmin, rmax, salt, full);
    float shadow = smoothstep(0.0, 1.5, lit - own - sh * 0.7);
    c = mix(c, under, floorc);
    c *= 1.0 - 0.38 * shadow;
    if (cr.top > 0.0) {
        c = mix(c, cr.col, cr.cover);
    }
    return c;
}

// How far a coppice coupe has grown back since it was last cut, 0..1: coupes
// are irregular panels of about 80 m, each on its own year of the rotation.
float coupe(vec2 pm) {
    vec2 q = pm / 80.0;
    ivec2 b = ivec2(floor(q));
    vec2 f = q - vec2(b);
    float best = 9.0;
    ivec2 id = b;
    for (int dy = -1; dy <= 1; dy++) {
        for (int dx = -1; dx <= 1; dx++) {
            ivec2 g = ivec2(dx, dy);
            vec2 r = vec2(g) + 0.1 + 0.8 * vec2(hash(b + g, 531), hash(b + g, 532)) - f;
            float d = dot(r, r);
            if (d < best) {
                best = d;
                id = b + g;
            }
        }
    }
    // Most coupes have grown back; only a few are newly cut.
    return sqrt(hash(id, 533));
}

// The wood far enough out that crowns are a pixel or less: stands of a kind,
// mottled at the scale of groups and of trees.
vec3 canopy_texture(vec2 pm, vec4 env, float rmean) {
    float pick = vnoise(pm / 150.0, 457);
    int sp = species(env, pick);
    vec3 c = leaf(sp, 0.5) * 0.85;
    c *= 0.88 + 0.24 * vnoise(pm / 35.0, 459) * soft(35.0);
    c *= 0.86 + 0.28 * vnoise(pm / (2.0 * rmean), 461) * soft(2.0 * rmean);
    return c;
}

// Downland: grazed turf, gorse and thorn, dew ponds and sheep folds. Far out,
// tussock marks in the old way of drawing rough grazing, so the turn to grass
// can be seen across the map.
vec3 downland(vec3 c, vec2 p, vec2 pm, float amount, float upland) {
    vec3 g = mix(vec3(0.60, 0.67, 0.41), vec3(0.62, 0.64, 0.44), upland);
    if (u_field_season == 3) {
        g = vec3(0.62, 0.62, 0.50);
    } else if (u_field_season == 2) {
        g = mix(g, vec3(0.66, 0.62, 0.40), 0.4);
    }
    g = turf(g, pm);
    // Grazed short in places, rank in others, at the scale of a flock's day.
    g *= 0.9 + 0.2 * vnoise(pm / 150.0, 447) * soft(150.0);
    g = mix(g, vec3(0.50, 0.55, 0.34), 0.35 * smoothstep(0.55, 0.8, vnoise(pm / 22.0, 449)) * soft(22.0));
    // Tussock marks, about eight to a cell, fading as the real texture comes in.
    float marks = smoothstep(2.0, 6.0, u_px) * (1.0 - smoothstep(0.25, 0.5, fine(30.0)));
    if (marks > 0.0) {
        vec2 q = p * 8.0;
        ivec2 b = ivec2(floor(q));
        vec2 o = 0.25 + 0.5 * vec2(hash(b, 471), hash(b, 472));
        vec2 d = (q - vec2(b) - o) * u_px / 8.0;
        // Three short strokes fanning upwards, in pixels.
        float tuft = 0.0;
        for (int k = -1; k <= 1; k++) {
            vec2 dir = normalize(vec2(float(k) * 0.55, -1.0));
            float t = clamp(dot(d, dir), 0.0, 3.2);
            tuft = max(tuft, 1.0 - smoothstep(0.35, 0.95, length(d - dir * t)));
        }
        g = mix(g, vec3(0.30, 0.40, 0.22), tuft * marks * 0.75);
    }
    vec3 out_c = mix(c, g, amount);
    if (amount <= 0.0) {
        return out_c;
    }
    // Dew ponds on the upland, about one to every 700 m square.
    {
        vec2 q = pm / 700.0;
        ivec2 b = ivec2(floor(q));
        vec2 ctr = (vec2(b) + 0.2 + 0.6 * vec2(hash(b, 475), hash(b, 476))) * 700.0;
        float r = 9.0 + 6.0 * hash(b, 477);
        float d = length(pm - ctr);
        if (upland > 0.3 && hash(b, 478) < 0.8 && d < r + 6.0) {
            out_c = mix(out_c, vec3(0.80, 0.77, 0.64), band(d, r, r + 3.0) * amount);
            vec3 water = u_field_season == 3 ? vec3(0.78, 0.82, 0.86) : vec3(0.42, 0.52, 0.55);
            out_c = mix(out_c, water, band(d, -r, r) * amount);
        }
    }
    // Sheep folds: hurdle pens on trampled ground.
    {
        vec2 q = pm / 450.0;
        ivec2 b = ivec2(floor(q));
        if (hash(b, 481) < 0.45) {
            vec2 ctr = (vec2(b) + 0.2 + 0.6 * vec2(hash(b, 482), hash(b, 483))) * 450.0;
            float a = hash(b, 484) * 3.1416;
            vec2 dir = vec2(cos(a), sin(a));
            vec2 rel = pm - ctr;
            vec2 l = vec2(dot(rel, dir), dot(rel, vec2(-dir.y, dir.x)));
            vec2 half_size = vec2(14.0, 9.0) * (0.8 + 0.4 * hash(b, 485));
            vec2 e = abs(l) - half_size;
            float box = max(e.x, e.y);
            out_c = mix(out_c, vec3(0.56, 0.52, 0.40), band(box, -1e3, 0.0) * 0.7 * amount);
            out_c = mix(out_c, vec3(0.30, 0.22, 0.15), band(box, -0.4, 0.4) * amount);
        }
    }
    return out_c;
}

// A hedge on the boundary of parcels Q, planted `age` years ago: older hedges
// are thicker, with more trees standing in them (Hooper's rule: about one
// shrub species a century), and a gate now and then. Its shadow and its trees'
// fall away from the light.
vec3 hedge(vec3 c, vec2 pm, Parcels Q, float age, float mosaic) {
    float old = smoothstep(50.0, 400.0, age);
    ivec2 lo = Q.a.x < Q.b.x || (Q.a.x == Q.b.x && Q.a.y < Q.b.y) ? Q.a : Q.b;
    ivec2 hi = lo == Q.a ? Q.b : Q.a;
    ivec2 pair = lo * 31 + hi * 17;
    float hw = (0.8 + 1.0 * old) * (0.8 + 0.4 * vnoise(vec2(Q.along / 9.0, 0.5), 491))
        + 0.3 * (vnoise(vec2(Q.along / 2.0, 1.5), 493) - 0.5);
    float gate = step(0.05, fract(Q.along / 170.0 + hash(pair, 495)));
    // Far out, a hedge is kept to a fine line so the field pattern reads.
    float w = max(hw, 0.35 * mpx());
    float on = gate * mosaic;
    float away = dot(-Q.n, SUN) < 0.0 ? 1.0 : 0.0;
    c *= 1.0 - 0.3 * band(Q.edge, w, w + 1.8) * away * on * fine(1.5);
    vec3 hcol = HEDGE * (0.8 + 0.45 * vnoise(pm / 1.1, 497) * soft(1.1));
    if (u_field_season == 3) {
        hcol = mix(hcol, vec3(0.36, 0.32, 0.27), 0.6);
    }
    c = mix(c, hcol, band(Q.edge, -1.0, w) * on);
    // Standard trees, more of them as the hedge ages, at no fixed spacing: each
    // stretch of about 40 m has one by chance, anywhere in it.
    // The look's tree size (R12) scales them and their spacing.
    float ts = u_look.z;
    float k = floor(Q.along / (40.0 * ts));
    for (int j = -1; j <= 1; j++) {
        float kk = k + float(j);
        if (hash2(pair, int(kk), 499) > 0.25 + 0.45 * old) {
            continue;
        }
        float tc = (kk + hash2(pair, int(kk), 501)) * 40.0 * ts;
        float r = (3.0 + 4.5 * hash2(pair, int(kk), 503) * (0.6 + 0.4 * old)) * ts;
        vec2 d = vec2(Q.along - tc, Q.edge);
        float dd = length(d);
        if (dd > 2.0 * r) {
            continue;
        }
        // Its centre, from this pixel: none on a road or a building (R27).
        bool first = Q.a.x < Q.b.x || (Q.a.x == Q.b.x && Q.a.y < Q.b.y);
        vec2 t = vec2(-Q.n.y, Q.n.x) * (first ? 1.0 : -1.0);
        if (!tree_clear(pm - d.x * t + Q.edge * Q.n, r)) {
            continue;
        }
        vec3 lc = leaf(hash2(pair, int(kk), 505) < 0.7 ? 0 : 4, hash2(pair, int(kk), 507));
        float sd = length(d + vec2(0.0, -0.6 * r * (away > 0.5 ? 1.0 : -1.0)));
        c *= 1.0 - 0.3 * band(sd, -r, r) * fine(r) * mosaic * (1.0 - band(dd, -r, r));
        float z = sqrt(max(r * r - dd * dd, 0.0)) / r;
        float lit = 0.6 + 0.5 * z;
        // None on a road (R23).
        float cover = band(dd, -r, r) * (u_field_season == 3 ? 0.7 : 1.0);
        c = mix(c, lc * lit * (0.86 + 0.28 * vnoise(pm / 1.3, 509) * fine(1.3)), cover * mosaic);
    }
    return c;
}

// The fields, meadows, closes and commons of the farmed land at real scale, with
// their hedges and hedgerow trees; old strips showing under grass where fields
// went out of cultivation. `far` is the colour of the farmland as drawn far out.
vec3 farmland(vec3 c, vec2 p, vec2 pm, float mosaic, float upland) {
    Parcels Q = parcels(pm, PARCEL, 401);
    Plot A = plot_of(Q.a, Q.sa);
    // Within a place's edge the land is its own closes, whatever parcel the
    // point falls in; the edge itself is hedged.
    float tl = u_town_show.x > 0.5 ? town_land(pm) : -1e9;
    if (tl > 0.0) {
        A.use = TOWN_CLOSE;
        A.hedged = true;
    }
    bool town_close = A.use == TOWN_CLOSE;
    // Within a place's edge its closes lie open, unhedged (R13; all of them
    // since R27, as hedges cut by the plots broke into fragments).
    bool open_ground = town_close;
    bool inner = A.use == CLOSE_GRASS || A.use == CLOSE_CROP || town_close;
    Parcels S;
    if (inner) {
        S = parcels(pm, town_close ? 60.0 : 70.0, 451);
        // Each small close is grass or under the plough, by its own lot; the
        // town's own are paddocks, orchards and gardens.
        if (!town_close) {
            A.use = hash(S.a, 453) < (A.use == CLOSE_CROP ? 0.7 : 0.25) ? CLOSE_CROP : CLOSE_GRASS;
        }
        A.angle += (hash(S.a, 455) - 0.5) * 2.0;
        A.h = hash(S.a, 457);
    }
    float strips = u_show.y * fine(14.0);
    vec3 col = c;
    float on = 0.0;
    vec3 grass = turf(grass_color(), pm);
    if (A.use == FURLONG) {
        col = furlong(A, Q.a, pm, Q.sa, strips);
        // Headlands where the plough turned, across the ends of the strips;
        // narrow grass balks along their sides.
        vec2 dir = vec2(cos(A.angle), sin(A.angle));
        bool ends = abs(dot(Q.n, dir)) > 0.5;
        float hl = ends ? band(Q.edge, -1.0, 6.0 + 2.0 * A.h) : band(Q.edge, -1.0, 0.7);
        col = mix(col, grass * 0.95, hl * fine(3.0));
        on = u_show.x;
    } else if (A.use == MEADOW) {
        col = turf(grass_color() * vec3(0.92, 1.05, 0.92), pm);
        // Hay: in summer half the meadows are mown, the hay raked into windrows
        // some 6 m apart.
        if (u_field_season == 1 && A.h < 0.5) {
            vec2 dir = vec2(cos(A.angle), sin(A.angle));
            float t = fract(dot(pm - Q.sa, dir) / (5.5 + A.h * 2.0));
            col = mix(col, vec3(0.76, 0.75, 0.48), 0.45);
            col = mix(col, vec3(0.66, 0.64, 0.38), band(abs(t - 0.5) * 6.0, -0.6, 0.6) * fine(6.0));
        }
        on = u_show.x;
    } else if (A.use == COMMON || A.use == SCRUB) {
        col = turf(grass_color() * vec3(1.05, 1.0, 0.95), pm);
        on = u_show.z;
    } else if (A.use == TOWN_CLOSE) {
        // A town's closes: paddocks of grass (most), orchards, kitchen gardens,
        // and on its fringe closes under crops, little fields of its own.
        // Among the houses, one open green (R13).
        float k = open_ground ? 1.0 : hash(S.a, 459);
        col = turf(grass_color() * mix(vec3(1.02, 1.02, 0.96), vec3(0.93, 1.03, 0.9), open_ground ? 0.3 : hash(S.a, 465)), pm);
        // Crops only on the fringe, away from the houses.
        if (k >= 0.62 && built_share(S.sa) < 0.02) {
            vec2 dir = vec2(cos(A.angle), sin(A.angle));
            vec2 nrm = vec2(-dir.y, dir.x);
            float t = fract(dot(pm - S.sa, nrm) / 9.0);
            col = crop_color(int(hash(S.a, 467) * 3.0)) * mix(1.0, ridge_light(t, 0.5, nrm), fine(4.5));
        } else if (k < 0.18 && fine(2.0 * u_look.z) > 0.0) {
            // An orchard: fruit trees in rough rows across the close.
            float ts = u_look.z;
            float row = 7.5 * ts;
            vec2 dir = vec2(cos(A.angle), sin(A.angle));
            vec2 rel = pm - S.sa;
            vec2 q = vec2(dot(rel, dir), dot(rel, vec2(-dir.y, dir.x))) / row;
            ivec2 b = ivec2(floor(q));
            vec2 o = vec2(0.5) + (vec2(hash(b, 461), hash(b, 462)) - 0.5) * 0.35;
            float d = length(q - vec2(b) - o) * row;
            float r = (2.0 + 0.9 * hash(b, 463)) * ts;
            if (S.edge > 3.0 * ts) {
                col *= 1.0 - 0.25 * band(length((q - vec2(b) - o) * row - SUN * 2.0 * ts), -r, r);
                vec3 lc = leaf(u_field_season == 3 ? 0 : 1, hash(b, 464)) * (0.8 + 0.3 * clamp(1.0 - d / r, 0.0, 1.0));
                col = mix(col, lc, band(d, -r, r) * fine(r));
            }
        } else if (k < 0.3) {
            // A kitchen garden in beds.
            vec2 dir = vec2(cos(A.angle), sin(A.angle));
            float t = dot(pm - S.sa, dir);
            vec3 bed = u_field_season == 3 ? vec3(0.46, 0.41, 0.32) : vec3(0.40, 0.54, 0.27);
            col = mix(vec3(0.47, 0.42, 0.32), bed, 0.5 + 0.5 * sign(sin(t / 0.8)) * fine(1.6));
        }
        on = 1.0;
    } else if (A.use == CLOSE_GRASS) {
        col = turf(grass_color() * vec3(0.96, 1.04, 0.94), pm);
        on = u_show.x;
    } else if (A.use == CLOSE_CROP) {
        // Ploughed in broad ridges ("lands") across the close.
        vec3 mean = crop_color(crop_of(int(A.h * 3.0)));
        vec2 dir = vec2(cos(A.angle), sin(A.angle));
        vec2 nrm = vec2(-dir.y, dir.x);
        float t = fract(dot(pm - S.sa, nrm) / 9.0);
        col = mean * mix(1.0, ridge_light(t, 0.5, nrm), fine(4.5) * strips);
        on = u_show.x;
    } else if (A.ghost > 0.02 && u_show.w > 0.5) {
        // Abandoned strips: ridge and furrow under grass, fading with the years.
        vec2 dir = vec2(cos(A.angle), sin(A.angle));
        vec2 nrm = vec2(-dir.y, dir.x);
        float t = fract((dot(pm - Q.sa, nrm) + 3.5 * sin(dot(pm - Q.sa, dir) * 0.0285)) / 15.0);
        col = c * mix(1.0, ridge_light(t, 0.4 + 0.6 * A.ghost, nrm), fine(7.0) * (0.4 + 0.6 * A.ghost));
        on = 1.0;
    }
    c = mix(c, col, on * mosaic);

    // Hedges: round closes, round the field block once it is established (R3's
    // fifty years), between holdings and round meadows.
    if (u_show.w > 0.5 && Q.edge < 40.0 && !town_close) {
        Plot B = plot_of(Q.b, Q.sb);
        bool fa = A.use != OPEN;
        bool fb = B.use != OPEN;
        if (town_land(Q.sb) > 0.0) {
            B.use = TOWN_CLOSE;
        }
        bool closes = A.use >= CLOSE_GRASS && A.use <= CLOSE_CROP || B.use >= CLOSE_GRASS && B.use <= CLOSE_CROP
            || (A.use == TOWN_CLOSE && B.use == TOWN_CLOSE && !open_ground);
        bool outline = fa != fb && (fa ? A.hedged : B.hedged);
        bool holdings = fa && fb && A.hedged && B.hedged && A.owner >= 0.0 && B.owner >= 0.0
            && abs(A.owner - B.owner) > 1e-4;
        bool meadow = (A.use == MEADOW) != (B.use == MEADOW) && fa && fb;
        if (closes || outline || holdings || meadow) {
            // A town's closes are kept by younger, trimmed hedges.
            float age = A.use == TOWN_CLOSE && B.use == TOWN_CLOSE ? 60.0 : max(A.age, B.age);
            c = hedge(c, pm, Q, age, mosaic);
        }
    }
    // Closes are small: each parcel of them is cut again into fields of about
    // 70 m, hedged all round.
    // A town's closes are hedged, walled or fenced; about a third of their
    // sides lie open.
    ivec2 sp = S.a.x + S.a.y < S.b.x + S.b.y ? S.a * 13 + S.b : S.b * 13 + S.a;
    bool open_side = town_close && hash(sp, 469) < 0.35;
    if (u_show.w > 0.5 && inner && !open_side && !open_ground && S.edge < Q.edge && S.edge < 40.0) {
        c = hedge(c, pm, S, town_close ? 50.0 : A.age, mosaic);
    }
    // The town's edge: a hedge where its closes meet the fields, with its
    // shadow on the side away from the light, broken now and then by a gate.
    if (u_show.w > 0.5 && abs(tl) < 6.0) {
        float g = length(vec2(dFdx(tl), dFdy(tl))) / max(mpx(), 1e-3);
        float d = abs(tl) / max(g, 0.3);
        float w = 1.2 + 0.6 * vnoise(pm / 6.0, 743);
        float gate = step(0.06, fract(dot(pm, vec2(0.0071, 0.0053)) + 0.3 * vnoise(pm / 90.0, 745)));
        vec3 hcol = HEDGE * (0.8 + 0.45 * vnoise(pm / 1.1, 747) * soft(1.1));
        c = mix(c, hcol, band(d, -1.0, w) * gate * mosaic);
    }
    return c;
}
