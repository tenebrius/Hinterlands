# Designer context — Hinterlands

Read this at the start of a session when acting as the **designer** for Hinterlands. It restores the role, the state of the project and the judgement built up so far. The brief itself is `docs/foundation.md`; this file is what sits around it.

## Resuming after a context clear (state at D134)

Read this section first, then `docs/foundation.md` from "Development resumes (D119)" to the end, which holds every decision since.

**State (see the status bullets below for later rounds).**
- Paused by Divish after the D132 report.
- The default model is D1:
  - stock-based demand for salt, iron and stone;
  - ports sell iron and salt;
  - golden `4a9737eafc305561`.
- Built but off: `port_reach` (D1b) and `cloth` (D2: the fallow flock, home wool, export cloth at the mills, the entropy split of wool, capacity `cloth_lb_per_w` 2.5, making cost 2.7 d, no duties).
- The gate (D120): 40 seeds a map, 30/40 per check, outside trade 3–50% (D126). Runs passing every check are reported, not gated.

**The last result (34de135, `docs/demand/d2d.md`).**
- The gate fails only over 2,000 on map 3 (22/40). Runs passing every check: 23 / 36 / 14.
- The cloth share is about 0.25 in 1400 (met). It is 0.19–0.22 in 1450 and 0.33–0.35 in 1500, against a record of about 0.5 and 0.7.

**Two decisions are waiting, in this order. My leanings are not yet decided.**
1. **The over-2,000 band on map 3.**
   - Map 3 keeps its town people: 0.37 of its people are in places over 2,000 in 1600, as in D1. But it has a bigger top place and smaller ranks 5–8, so the count falls below 5 in 4 of 10 runs.
   - The band (5–12, from v1) has no source, and D1 passed it only with the artefact iron towns.
   - Leaning: judge the band against a sourced urban hierarchy (England in 1300 and 1600: the urban share and the number of towns of 2,000+ per head; Dyer, Campbell, Wrigley), or replace the count with the urban share. Decide on the sources, not on the run; rule: revise only with a reason that stands alone.
2. **The cloth pace after 1420** is half the record's.
   - Candidates: capacity growth tied to the mill towns' W is too slow; or the cloth price, which is flat at Munro's 1371–1450 value while English cloth prices rose after 1450.
   - First ask for a trace (capacity against use by decade, mill W growth, netbacks of cloth against wool). Do not tune `cloth_lb_per_w`; it is calibrated at the start only.

**After those two:** gate D1b and D2 as the default, then D3 (wealth raises stocks; cloth per head rises with income, with Dyer's three yards in 1500 as its check). Growth after 1480 is still noted.

**Working rules, beyond the ones above.**
- Every pre-registration names its mechanism and says which quantity that mechanism fixes and which it leaves free (rules 5 and 12). The implementer confirms the mechanism is in the code before the run.
- A calibration may fix a start or a level, never the path that is the test (D130, D132).
- Never pre-register a gain inside the run-to-run noise (rule 11). At 40 seeds a count moves by about ±3.
- **Decision numbers:** the implementer writes D-numbers in `docs/decisions.md` for builds (D125, D127, D129, D131, D133). The designer takes the next free number in `docs/foundation.md`. Check both files before numbering.
- Each round: pull main, read the implementer's report doc, record the decision in `docs/foundation.md`, update this file, commit and push to main and the branch `claude/sharp-lovelace-azqxcu`, then send one message to the implementer naming the commit.
- The proposal session (`session_01MrMcFJCCYku1zTBbBkDVfp`) sends owner proposals (`docs/proposals/`). Decide them in the foundation and reply "no reply needed".

## The role

There are two roles on this project and they are kept separate.

- **The implementer** builds, tunes and validates. Works autonomously on `main`, no CI, no PRs. Records every milestone and every model change as a numbered decision (`docs/decisions.md`, D1…). Raises only the points listed in the working agreement.
- **The designer** (this role) owns the model. Decides what the simulation must do, judges validation and real-terrain results, corrects the specification when evidence shows it is wrong, and keeps scope from growing. Does not write production code. Writes the brief, the checks, and the reasoning behind each change.

The owner is Divish. He has had projects of this kind fail before from over-ambition, so the standing instruction is: **small, proven, gated**. Every mechanism comes from a recognised published model where one exists. Nothing from a later version is started before the current one passes its checks. When in doubt, cut.

**Divish, 2026-10-22: v6 is not the end.** Underlying mechanics that are not a real trade economy break the fun. The model must become one (v7, marketplaces).

**Divish, 2026-10-22, on v7 (D91):** marketplaces go in the core, not behind a switch (tag `v6` first). Don't give up easily: no stop rule; keep diagnosing and fixing, but never by loosening the tests. The 50 ms budget is dropped; time is reported.

**Design authority (Divish, 2026-10-21).** The designer has full control of the design requirements: defaults, the acceptance test, scope, versions and the order of work. The aim is a **semi-realistic but satisfying** simulation that keeps moving forward; realism is a means, not the goal. So:
- Decide, don't escalate. Record every such decision in `docs/decisions.md` or `docs/foundation.md`, and tell Divish of major changes of direction.
- Weigh each round by what it adds to the simulation someone watches, not only by what it adds to a score. A real-terrain miss is worth chasing only when the fix improves the model in general.
- "Small, proven, gated" still holds: it is how the project keeps moving, not a brake on it.

## Where the project stands

- **v1 released**, then **v1.1 released** after the Kent real-terrain test. Validation passes on all three generated maps at the 75 % pass-rate rule. Golden determinism hash at v1.1: `68beae826cf800cd` (same natively and in the browser). One year runs in about 21 ms natively, 28 ms in the browser, within the 50 ms budget.
- **v1.1 defaults:** market size `W` as its own state (the published Rihll–Wilson variable), income from handling trade, hamlet founding scored on food and water only, brook-scale water supply (a 5 km² brook or the coast within 3 km). The river-corridor check is reported, not required.
- **Kent (lower Thames) is the real-terrain test case.** 128 × 128 km at 1 km from SRTM, 70 real towns of c. 1300 as ground truth, scored over 20 seeds. Findings so far:
  - The model places ports and river towns and towns over 1,000 with real skill. Largest city at Gravesend (tidal Thames) in 15 of 20 seeds under v1.1.
  - Terrain and soil carry **no signal** for where the small inland towns sat (feature scores 0.20–0.48 against real sites, 0.5 being none). Their sites came from roads, market spacing and lordship. So for towns under about 1,000 the yardstick is count and spacing, not placement. Real Kent: 29 inland places over 500, 10.6 km apart; v1.1 gives 20 at 9.1 km.
  - SoilGrids and a valley-floor proxy do not improve placement. Uniform soil stays the default; both are options in the importer.
  - The standard placement metric is **AUC against lowland cells**, which cannot be fooled by "towns sit low". Always include a random baseline.
- **Roman roads test failed (D32).** Itiner-e roads (58 segments, CC BY 4.0) as an initial road set: Rochester 1/20, Canterbury 0/20, settlements on or next to a road 20% (land 32%, real towns 47%), Gravesend largest in only 6 of 20. Causes: the shadow is in travel cost, so roads make neighbours suppress each other; nothing rewards a road site; a road crossing a river pays the full crossing cost because the model has fords but no bridges. The expectation rested on the crossings being special, which the model never made them: my error.
- Roman roads are the **first Kent input with signal for small towns** (47% of real towns against 32% of land). The model moving settlements off roads is a model fault.
- **Roman bridges test failed too (D33).** 52 bridges, 30 on navigable rivers; Rochester 1/20 (bar 5), usually no settlement near the bridge at all; Canterbury bridged only on a stream. Roads and bridges stay Kent options, not defaults (Gravesend 15 → 8 with roads).
- **`shed_unfed` (D34, ca0b5df):** v1.1 deleted about 170 towns of 1,000+ per generated run. With shedding, every per-seed check improves (19/20/18 runs pass everything, against 7/13/14), failure rate 0.55–0.59. The gated "largest city varies" check failed (maps 2–3); it had only passed through the defect.
- **v1.2 decided (mine):** `shed_unfed` is the default; the contingency gate moves to the middle tier (mean top-10 overlap between seeds ≤ 0.80; largest-city variation reported); Kent's standard configuration is uniform soil + Roman roads + bridges (53% of places of 1,000+ on a road, real 47%); Rochester is closed as a target; Kent becomes a regression report. `docs/foundation.md` already describes v1.2.
- **v1.2 released (c03d31e):** validation passed on all 3 maps; top-10 overlap 0.36/0.40/0.37 (gate ≤ 0.80); runs passing every check 19/20/18; golden `2b00434da9781880` native and browser; 26.1 ms/year wasm. Kent regression: AUC 0.49/0.53/0.46, 53% of places of 1,000+ on a Roman road.
- **v2 briefed** (`docs/foundation.md`, "v2 specification"): salt, iron, stone beside grain; fixed exchange ratios, decay from value density (`β_g = β/ρ_g`); production-constrained goods flows paid in grain from the marketed share, so no food is created; iron fuel-bound (Weber); specialist hamlets (`Mine`, `Saltern`, `Quarry`) founded on resource cells with reachable demand; all behind `goods_enabled`. Stages G1–G4 run on without approval unless a check fails; gate: v1.2 checks, ≥ 2/3 of sites producing, no specialist largest, < 50 ms wasm.
- **G1 done (1317aa6, D38):** held sites all produce; forest within 5 km of iron sites 0.17 against a land mean of 0.66 by year 200; `v_g` = 1, `fuel_per_iron` 0.1 accepted. Only ~38% of sites have a holder, so the ≥ 2/3 gate rests on G3 specialists. D36–D37 are the rendering merge from the graphics session.
- **G2 done (D39):** payments balance (goods 53,226 received, 52,405 paid, 1.5% short where buyers hit their marketed share); median route cost iron 19.7 > salt 14.5 > grain 13.3 > stone 9.0, full order in 22 of 24 runs. Accepted.
- **G3 gate failed (a362721, D40):** salterns at the 1,000 cap became the largest place in 18/20 (map 1) and 11/20 (map 2) seeds. Spec error (mine, rule 7). Corrected to the shopping model with capacity scaling (D41); one re-run of the gate, then G4 if it passes. Forest near iron sites 0.00 to look at in G4.
- **Gate re-run with the shopping model (0c326ed, D41):** oversupply gone (payment share 3%), but the largest place is a "saltern" in 19/20 (map 1) seeds: salterns founded on the best harbour sites grow into the city and keep the label. Decided (D43): the kind follows the economy (specialist only while goods sales ≥ 25% of inflow), and the gate tests that. Map 3's over-5,000 median (1) is for the implementer to tune. D42 is reserved for the graphics merge.
- **v2 gate after D44 + re-tune (85b2003):** all gated checks pass except map 3's over-5,000 median (1). Decided D48: that median becomes reported (v1.2 passed it by one seed; per-seed hierarchy checks carry the purpose). Re-tuned values (α 1.088, K 3.39, ε 0.137, β 0.139, market share 0.487, stone demand 0.016) become v2 defaults. G4 next.
- **v2 released (1bcbb39, D49):** validation passed; golden `b0de71500774acf8` (28.2 ms/year wasm); goods layer and panel in new files; screenshot `docs/screenshots/v2-goods-year450.png` reads well. Coppice rule for iron fuel approved for v2.1 (floor 0.3, output capped by regrowth above it), with a gate re-run.
- **v2.1 done (36da3c5, f76045c; D50–D51):** coppice on: iron output 18–28 a site, forest near iron 0.17–0.22 against land 0.44–0.46; v2 gate passed (19/20/20 runs pass everything); golden `f67bf8565c500fda`, 25.9 ms/year wasm. Kent's 17 real sites: all 6 Wealden bloomeries sell in 20/20 seeds; Faversham, beside the Seasalter–Graveney pans, now the largest in 10 seeds. v3(a), the shadow in km, briefed with a pre-registered Kent expectation (on-road share ≥ 32%).
- **v3(a) first attempt failed (9a81e76, D52):** km shadow behind `shadow_km`; best tune fails market spacing on maps 1 and 3 (radius vs drain trade-off). Kent met the expectation (39% on roads). Decided D53: separate drain radius (~8 km) from the founding shadow (~15–18 km); if that fails, drop v3(a) and go on to (b). The spacing check is not touched.
- **v3(a) dropped (af66229, D53):** two radii tested worse; in km form the drain is what spaces markets. The cost shadow stays. v3(b) roads with condition and slow decay specified (gate: v2.1 checks plus road length stable within 10% from year 300 to 500).
- **v3(b) roads passed (9cb9fb4, D54):** road length change 300→500 of 2–4%, 40% of road cells abandoned over a run; golden `0b590b3b017b63d0`. Kent: 19% of Roman road cells kept, on-road share 29%, inland >500 now 12 at 10.8 km (spacing right, count low), 445 settlements. Watling Street fading needs the outside world (v5). (c) bridges and (d) mills specified, then the v3 milestone.
- **v3 released (6541ce5, D55–D56):** bridges 14–17 a run, places of 1,000+ near bridges 11–44% against 1–2% of all settlements; mills on 38–41% of settlements (any stream field gives a mill; `mill_area` never binds); golden `171ec4461a912a1d`, 23.7 ms/year wasm; screenshot `docs/screenshots/v3-year450.png` good.
- **v4 briefed** (`docs/foundation.md`, "v4 specification: shocks"): Historical calendar (year 0 = 1100; famine 215–217, plague 248 with recurrences 261 and 269), plus Random and Off modes; famine, plague, harbour silting (instead of river shifts, since `Terrain` is fixed), planted towns. Gate: v3 checks at year 500, a ≥25% fall after the plague, recovery to ≥90% by year 500, low-yield places failing more, top-10 overlap with the no-events run ≥ 0.5. Stages E1–E4.
- **v4 E1 (588d0ed, D57–D58):** plague drop 0.36–0.37, recovery 1.66–1.80 of the pre-plague peak (fast), top-10 against Off 0.80, desertion failed on map 3 (10/20) because the urban drain near cities on good land confounds it. Decided D59: measure desertion beyond 8 km of places over 2,000; D58's readings (stability leaves out event years; local yield) confirmed; report the trough length. The post-plague rule (Dyer) is held in reserve.
- **v4 E1 re-run with D59 (ec149e0):** desertion still 11/20 on map 3 (failure 0.18 against 0.15). The model has no move from poor land to vacant good land. Decided D61: the implementer's resettlement rule (30 years after a plague, low-yield places send 2% a year to the partner with the most free food per head). Plague total regained after 48–50 years, and year 500 equals the Off run. If the trough looks too short in the viewer, the lever is more historical recurrences in the calendar (1375, 1390, 1407, 1438, 1464, 1479), not a slower growth rate.
- **v4 E1 passed with D61 (8488d97):** desertion 20/20 on every map (lowest-yield quarter fails 0.17–0.23, highest 0.00); plague drop 0.36–0.37; total regained in about 45 years; year 500 at 1.04–1.05 of the Off run. Map 3 margins are thin (largest share 16/20, settlements 17/20; runs passing every check 11/20): watch at the milestone. E2 silting being gated.
- **v4 E2 (fd5a661, D60):** the busiest shallow port was the largest city in every run, so silting at cost 3 reshuffled the top and failed map 1 spacing; the implementer passed it at cost 1.0, where map 3's port never loses rank. Decided D62: leave out the largest settlement, put `silt_cost` back to 3, and gate together with E3.
- **v4 E2 + E3 passed (604e843, D62–D63):** with the capital left out, the silted port falls from rank 5 to 10–12 within a century (lost rank in 18–19 of 20 runs). Planted towns from year 150 (planting earlier redirected the whole hierarchy); survival 57–82%, accepted. E4 and the milestone under way.
- **v4 released (48e6777, 7d86233):** Historical events on by default; validation passed; golden `2c5e9c5c90ab1d43`, 27.6 ms/year wasm; screenshots v4-year450 and v4-plague-1349. Kent: on-road share 30%, inland >500 10 at 11.9 km, largest Gravesend 13.
- **v5 decided as the last version (mine):** outside ports (fixed grain demand, sole seller of imports) and ships improving 1300–1500 (sea cost −30%); joined regions and the rest of the old v5 dropped. Then a release pass, with no new mechanisms.
- **v5 O1 (cac85b0, D64–D65):** outside trade at 5% passes everywhere; map 1 spacing 13/20 (chronically marginal). Decided D66: one re-tune of the founding and shadow values with ports on; if that fails, record an exception; the measure stays.
- **v5 O1 passed after the D66 re-tune (81d3c1d):** shadow_weight 4.53, found_threshold 0.449, noise 0.324; outside trade 5.1–5.5%; map 1 spacing 15/20, exactly at the line (watch it); golden `8ee85c10266b1dd2`. O2 (ships) being gated.
- **v5 milestone passed (302b529; D65–D67):** outside trade 5.2–5.3%; golden `4ce58fae34550933`, 26.2 ms/year wasm. Ships lift the coast only on map 3 (+6 points); the coastal share falls from 1300 either way. Kent: Watling Street 24% still road at 1600, so Dover trade did not save it. My expectation was wrong: the model carries Continental goods by sea, which is largely true historically, while the road's persistence came from traffic the model does not have (people, the king's messengers). Release pass approved: README, parameters.md, a "what you are watching" panel, two view fixes (sea lanes bundled, label collisions), final screenshots, freeze.
- **v5 released and frozen (b9ca4de):** golden `4ce58fae34550933`, 27.4 ms/year wasm, validation passed (14/17/17 runs pass everything). README rewritten, parameters.md regenerated, "What you are watching" panel added, sea lanes bundled, settlement names take label priority; R8 merged (D68). Release screenshots in `docs/screenshots/release/`: the coast now reads cleanly; feature labels still cross each other, and that is noted in `docs/rendering.md` for the graphics session. v6 S0 under way.
- **v6 S0 (9ac3ec7, D69):** shadow prices deterministic; +15 ms a year in wasm (44.9 total). Clark `b_P` and R² in range, `b_Y` unidentified (no local weather); prices nearly uniform; the post-famine year converges poorly. Decided: S1 (goods by equilibrium, grain numeraire, ≤ 45 ms wasm) approved; W1 local harvests (a correlated yield noise field, ~15 km, sd ~15%) to come before S2.
- **v6 S1 first gate (b994d39, D71):** map 2 spacing 14/20, because iron at 22–80 grain made mines of 1,500–2,800 people (demand set before coppice). Decided: re-scale `demand_iron` so the median town price is 1–3; one re-run of the gate.
- **v6 S1 second gate (cabdac6):** iron at 0.0013 fixes spacing (specialist medians back to 106–334); map 2 iron still dear (5.6–6.9), accepted. Sites producing failed (14/17/14), because the founding test counts quantity. Decided D72: count value at last year's buyer prices, so that dear goods draw new works. One more re-run.
- **v6 S1 third gate (d31e04a):** only map 1's sites-producing check fails (14/20): remote sites idle while iron sells at about 1.2. Decided D73: under prices, idle marginal sites are the equilibrium (the extensive margin), so the gate becomes at least one working site per good, with the share producing reported. Then market_goods becomes the default, and W1 local harvests is briefed (σ 0.15 on a 15 km field, fresh each year).
- **v6 S1 default (28ddc53):** market_goods on, golden `ec7a24a7caeac414`, 31.9 ms/year wasm. **W1 (4d35a23, D74):** map 1 top-10 against the Off run fell to 12/20, because the Off run gets different weather. Decided D75: draw the weather up front from the one Rng so on and off runs share it; set the sd at cells to 0.15 (it came out 0.095); one re-run. `b_Y` is identified but tiny.
- **W1 second gate (2c066b6):** failed widely; noise with asymmetric losses shrinks the early region. Decided D76: park W1 until after S2 and S3 (my error, rule 8); order S2 → S3 → W1; S2 starts with a design note (supply and demand, value unit and budget, W premium, what replaces food_cap, famine convergence, cost), reviewed before code.
- **S2 design note (d1faf0c) reviewed, D77:** approved, with rents in value (towns' real income), fees paid by the buyer outside the routing price (to avoid counting break of bulk twice and starving the ports), p_floor 0.25, c_max 1.2, the shadow decoupled in the first commit, the convergence fix, and a hard 50 ms wasm limit (45 target). S2 build approved: gate v5 checks plus Clark b_P and R²; stop and report.
- **S2 first build (7b00639, D78):** solver fine, but no towns form, because rent grain has no buyers and K was tuned on burned grain. Decided D79, option C: K becomes a real purchase (Wilson's facility cost; malt, fodder and households, Campbell et al. 1993), with total market purchase K × W including people's food; market people don't farm; one tuning round, then the full gate.
- **S2 stopped (bf77fcd, D80):** with K as a purchase no towns form on any map (hierarchy checks 0/20); priced grain breaks the free-inflow feedback the hierarchy rests on. The stop rule applied: market_grain stays off, S1 stays. Replaced by S2′ grain relief: surplus above c_max goes to deficits by the shopping model, unpaid, not counted toward W. Then S3 stores, then W1. Divish told.
- **S2′ relief passed (8875174):** with shadow_drain 0.025, deficit met 86–88% in normal years but 6–8% in 1315–17; the famine dip is unchanged. Becomes the default. S3 stores briefed (D81): own store first, relief from neighbours' surplus and stores, fill to 0.5 × need, 8% loss a year; pre-registered famine dip ≤ 0.18; granary view.
- **S3 stores passed (b88f75f, D82):** 1.0 / 0.05; dip 0.17–0.18 (target met); stores 0.49–0.52 before 1315 (map 1 just over the band, accepted); famine deficit met 55–59%. Default. Stores never refill without weather, so W1 is re-gated next with buffers on (pre-registered: population at year 200 within 20% of the no-weather run; stores ≥ 0.2 at year 400). Then the v6 milestone, and S4/S5 decided.
- **W1 third gate (1928344):** failed (population at year 200 ≈ 0.3 of the no-weather run; stores never used for growth, only below need). Decided D83: top up from stores to food_cap × need (consumption smoothing); one last re-gate; if it fails, W1 is dropped for good. Either way, the v6 milestone follows.
- **W1 dropped (D83 fourth gate):** smoothing from stores changed nothing; food is no longer the constraint. Variance drains people through the market term of D15: positive pull is capped and moves only migrants, while negative pull removes people. A known limitation. If weather is ever wanted again, the market term must first be made symmetric (option B). Not scheduled. The v6 milestone is running.
- **v6 closed (f12337e, add36a7):** defaults S1 + S2′ + S3; golden `0e65d09ec147b5c7`, 30.3 ms wasm; famine dip 0.17–0.18. D84: S5 dropped, S4 not scheduled. Divish approved one weather attempt (W2, D85): pull on the 10-year average inflow; step 1 without weather, step 2 with σ 0.15; drop for good if it fails. Divish also wants to revisit S2.
- **W2 weather passed (7d7173e, D86):** smoothing the pull fixed it (population at year 200 0.95–0.96 of no-weather; met). The stores refill expectation was missed (0.02–0.03 at year 400); recorded. Weather and smoothing are now the defaults. Next, if Divish agrees: a zero-risk diagnostic of turnover-based markets (grain sold through marketplaces), before any S2 redesign.
- **v7 marketplaces planned** (`docs/foundation.md`, "v7 plan"): grain sold through markets by seller routing (which keeps the RW feedback), bought by price, market size on turnover (RW's D), market people fed by real income (tolls, rents, fees, goods). M0 zero-risk diagnostic approved (turnover concentration against market_in; town break-even toll); then M1 note, M2 build, M3 stores trading.
- **v7 M0 passed (8c6aa27, D87):** turnover ranks like market_in (Spearman 0.96–0.98); 91–93% of towns over 2,000 break even at a toll ≤ 10%. Rents carry the towns (consumer city). M1 note briefed: a two-level market (villages ↔ their market by gravity; markets ↔ markets by price), W on turnover, market people fed from income, no establishment purchase.
- **v7 M1 note approved (5420934, D88):** two-level market; rent grain is the lords' income and their households buy it; market_min 50; deficit villages pay from own sales; village stores and relief stay on in M2, market stores in M3. M2 build approved: untuned run first (pre-registered: hierarchy checks pass on ≥ 2 maps), one tuning round, gate with v6 checks plus Clark per seed, then stop.
- **v7 M2 first build failed (20db863, D89):** W on grain sold kills the feedback. Decided D90 (my error, rule 9): W grows on grain arriving (food supply draws people); K re-read as consumption (1.2–2.0), tuned with market_share; new gated waste check (unsold ≤ 10% of arrivals after year 200) separates this from P3. Re-run untuned at K 1.5 with market_share × 1.5/3.39, then one tuning round, then the gate.
- **v7 M2 second build (4052d01):** with low K every village's home sales grew W to the whole population; nobody farmed and the region collapsed. Decided D92: only arrivals from other places grow W (Christaller's centrality, *Bedeutungsüberschuss*), and W is capped at pop minus field labour; farmers own their sales revenue. Re-run in the core.
- **v7 in the core (d8c85dc, interim):** gravity grain push retired; golden in progress d370f1235a22cc2c. Market people buy K = 1.5 per head and eat 1 of it (the rest is the establishment). Best so far (K 1.5, market_share 0.4, toll 0.10, market_min 100), map 1 seed 1 at year 500: largest 6,763, 5 over 2,000, 3 over 5,000, slope −0.91, unsold 3%; population 77k against v6's 112k. The `v6` tag could not be pushed (proxy 403): v6 = commit 4052d01; Divish to push the tag from a normal clone.
- **v7 M2 PASSED (cc8f7ba, D93):** the model is a trade economy (grain sold at markets by price, towns on rents and tolls, unsold 1.4–2.1%). Clark `b_Y` −0.024 to −0.033 against his −0.025. The untuned pre-registration failed (recorded). Watch: weather costs growth (0.68–0.70), outside trade at its low edge. M3 market stores briefed (D94; competitive storage, Deaton & Laroque): pre-registered dip ≤ 0.16 and price autocorrelation ≥ 0.3.
- **v7 M3 so far (f8e1c29):** stores barely act (trend), store_spike accepted; villages have no money, so stored grain can't reach them (famine deficit met 0–3%). Decided D95: village purses (farmers keep sales revenue, value units, 10% a year loss, cap 1 × a year's need), the first value stock; credit kept for later. Divish had asked whether villages have money.
- **D96 (Divish):** everybody holds money, not only villages: farmers' and markets' purses (lords, church, merchants; works included); coin enters through port exports and leaves through imports and untraced spending, so the region has a money stock and a price level (Mayhew 1995). Floating money is no longer "never planned". A one-page money design note first (conservation unit test; price-level check 0.4–2.5).
- **Money note approved (4b4a1d0, D97)** with one change: untraced spending recirculates inside the region (farmers to their market; markets back to their suppliers, 20% of it out as luxuries), and the true leak is only 1% coin wear. The price level is anchored by the external balance (Hume's price–specie flow) against the ports' world price.
- **Money first build (f0566d6, D98):** conservation exact, Clark passes, unsold 0%, price autocorrelation 0.48–0.51 (Clark 0.48). Misses: dip 0.21–0.22, deficit met 0.24–0.27. Decided D99: the establishment's grain demand gives way first in dearth (ε_est 1.5; historical malting bans); stores and purses tunable within Clark's ranges; dip and deficit are design targets, not gates; price check floor becomes half the world price (0.3), revised after seeing early export-parity prices.
- **v7 M3 PASSED (5550243, D100):** money, stores and purses in the core; golden `ff58d1b63e4c56d2`; autocorrelation, deficit and carry-over targets met; famine dip 0.22 missed. Against history (yields −40/−60/−10%, mortality about 10–15%), the model's famine is too lethal; the cause is D34 shedding for any shortfall. M4 approved (D101): hunger tolerance 0.85 and the historical famine shape; pre-registered dip 0.10–0.16 and weather cost ≥ 0.85.
- **M4 first build (47c7f41):** tolerance 0.80 gets dip 0.16–0.17 and weather 0.84–0.87, but spacing (map 1) and roads (map 3) fail, because marginal places survive. Decided D102: growth and chronic shedding on the 5-year average food ratio (Wrigley & Schofield), acute shedding below tolerance on the current year, the larger applies; tolerance stays within 0.8–0.9.
- **M4 second build (9646f15):** spacing and roads pass, but the dip worsens to 0.24, because the averaged ratios count the famine twice. Decided D103: chronic measure = mean₅(food) / need_now (food against the people there now); weather 0.80–0.83 recorded as a miss unless this moves it.
- **M4 third build (8d62c19):** averaging food over current need lags growth (region at half speed; dip 0.10 only because the region is smaller). Decided D104: drop the five-year averaging; return to tolerance 0.80 on the current year; restore spacing through the shadow drain (spacing belongs to market competition, not to hunger culling), with road_keep_load tunable if needed.
- **Wool and land use (D105):** Divish's proposal (`docs/proposals/wool.md`) accepted with all his positions: after M4; Option A (pasture on spare land) first; wool doesn't grow W at first; income split by rent share; murrain at L2; cloth not scheduled. Addition: wool parameters sourced (Campbell 2000; Lloyd 1973) before L1. L0 diagnostic briefed, to run after M4.
- **v7 M4 PASSED (36fa76a, D106):** hunger tolerance 0.80 on the current year; shadow_drain 0.05; road_keep_load 600; golden `e788381d55fa67d2`. Weather met (0.85–0.90); dip 0.17–0.18 (a near miss); market stores idle (carry-over 0.00), a regression to revisit; map 1 runs passing every check only 8/20. L0 wool started.
- **Wool L0 (bf28f36), D107:** A wins (labour-limited, pasture doubles after 1348); B starves the towns (not met). L1 approved: A with lowest-arable-rent pick, rho_wool 50 (sourced), £5 a sack (Lloyd), pasture_labour 5 a cell (calibration), D105's gate; in the core.
- **Wool L1 first build (ba2f1fe), D108:** graze land not worth ploughing, shepherds in the labour guard; the plague recurrences 1375–1479 added to the calendar (Hatcher 1977, approximate) so the depopulation lasts to about 1500; wool counts as outside trade; drain tunable for spacing.
- **Wool L1 second build (8659b32), D109:** every L1 gate passes; wool 55–62% of coin before 1348; only outside trade fails at 1600 (0.23–0.27). Decided: apply Clark's −25% wool price after 1350 first, then revise the band on the numbers if needed. Hatcher's years after 1390 unverified; population in the 1400s too high against Hatcher (a fall of a half to two-thirds), noted for after L1. Idle market stores next after L1.
- **Wool L1 third build (a2c6f26), D110:** outside-trade band revised to 3–30% now that it includes wool (stated reason); map 1 over-2,000 at 14/20 must be fixed, not waived; robustness low (runs passing every check 6/6/2), so a consolidation re-tune after the stores fix; Kent pasture missing is a data gap (no chalk or marsh soils).
- **Wool L1 PASSED (f1ad38c, D111):** market_share 0.35; golden `1ebe5979ca2bf8fe`; robustness 15/15/7; stores carry again, so the store fix is cancelled. D112: L2 murrain (0.7 in 1315–17, 0.85 in 1318–22, approximate) and L4 view next together; then a population check against Hatcher; L3 parked.
- **L2 and L4 PASSED (3c19458, D113):** golden `7d534fccaae58528`; murrain reshaped after Slavin (×0.85 in 1315–22, ×0.9 in 1323–32). The Hatcher check failed (1450 = 1.15–1.19 × 1340 against 0.33–0.50). D114: endemic mortality era 1349–1480; pre-registered 1450/1340 within 0.40–0.65; v4 recovery check to be revised to the historical range if it fails.
- **D114 PASSED (a92c5bb):** endemic mortality 0.015; 1450/1340 within 0.45–0.64 (met, 60/60); golden `b4f78a0e4adf502b`. Noted: 1600 population too high (fast r_grow after 1480); carry-over peaks at 0.44. D115: v7 release pass (README, parameters, watching panel, prices, purses and stores visible, screenshots, freeze).
- **Kent geology (73bfde3), the last change:** BGS chalk ×0.6 and marsh ×0.35; pasture on the Downs 0.53 and the Marsh 0.60 against the Weald 0.10 (pre-registration met); small-town AUC 0.50 → 0.57, the best yet. map-geology.json becomes Kent's standard setting.
- **DEVELOPMENT RESUMES (Divish, 22 October 2026; D119).** Order: R, the robustness re-tune (pre-registered in foundation: maximin of all-check runs, every gated check at 15/20 or better, a screen of 6 levers × 4 values, a joint grid, then confirmation on seeds 21–40 against the current setting; expect 12 or more on every map), then D0–D3. Stages are switched off until their gate passes, then they become the default in the core.
- **R done (a7d6943), D120:** current setting kept (`k_maint` 1.2 led on seeds 1–20, lost on 21–40); the pre-registered ≥12 missed (my error 11). The gate moves to 40 seeds (30/40; no-worse rule where the accepted model is short: road length map 1 at 29/40). D0 sent: diagnostic, no feedback, sources per D105, pre-registered iron falling after 1348 and stone in growing towns. Road persistence is the mechanism candidate if road length is still weak after D2.
- **D0 done (39b3dbb), D121:** both pre-registrations met. Stock iron is 30–40× today's: 0.0013 was fitted to coppiced supply (D71), not a demand; the missing piece was outside supply. D1: (1) ports sell iron and salt at price 1 like imports (Childs 1981; Bridbury 1955), built first at today's demand; (2) stock demand for iron, salt and stone, switch `demand_stocks`. Pre-registered: specialist median under 500, iron price between 1 and port parity, outside trade inside 3–30% (not revised before seen). The cloth note may come with D1 (D0: home cloth needs 3–5× the wool at Dyer's 1500 figure).
- **D1 done (c96bd73), D123:** accepted. Gate passed on 40 seeds; all-check runs 27/28/17 (reference 19/27/16); golden `4a9737eafc305561`; ports supply 80–90% of iron. Traces before D2: map 1 specialists by good (salt towns acceptable), iron-price outliers 16–21, store carry-over ~0, map 3 outside trade. Cloth decided: 1.5 yd constant (D3 gives the rise), one cloth good by wool content, fallow flock as step 1, ports buy export cloth, and the wool subsidy (~1/3, 1340s) against the cloth custom (1347) as the wedge. Pre-registered: cloth share of wool under 0.4 in 1300–40 and over 0.6 in 1450–1500; at least half of cloth from W 100–2,000.
- **D123 traces (8ed969e), D124:**
  - Map 1's large specialists are iron mines in districts no port reaches, selling at 20–30 (my D121 fault: no carriage beyond reach). Fixed by option (a), an arc from every settlement to its nearest port at full travel cost for outside-traded goods (D1b, gated, pre-registered specialist median under 500). Wool and export cloth use the same rule from D2.
  - Stores: the measure lost its sample; re-read on the cheapest decile against trend; no claim ahead of goods.
  - Accepted: fallow flock (3.6 lb a person in 1300) and the cloth prices and duties (Munro, Barnes).
  - D1b then D2, reported together.
- **D1b (95d6737) failed the gate, D126:**
  - Pre-registrations met: specialists 79/47/33, inland iron 1.06.
  - Outside-trade ceiling re-founded at 50% (scale: a county-sized region is not a nation, Alesina et al. 2000; the definition now holds every sourced channel; 'export colony' means more than half).
  - D1b and D2 are gated together, since D1b removed artefact middle towns and cloth is the mechanism for real ones.
  - The cloth-share pre-registration is built in by the 3.2 d making-cost bracket, so it is now reported. The independent test is a sourced wool share of broadcloth value against the 2.62–3.77 bracket.
- **D1b + D2 (94b2e8c) failed, D128:**
  - All-check runs 6/5/4; spacing too wide, rank-size flat, recovery low on map 2.
  - My spec error 13: home cloth made at mill towns transferred everyone's clothing bill to them, against D118.2.
  - Corrected: home cloth is made where it is worn (wool demand at home); the chain is export cloth only; wool's outlets are split by the entropy form (share 1.00 is too sharp).
  - Munro's shares put the making cost below the bracket, so 3.2 d stands in for a pre-1340 quality gap.
  - Exports per head 10 d against England's 14 d weaken the scale reason for the 50% ceiling; it stands on the export-colony reason.
  - Attribution run (wool without mills) first, then the corrected D2.
- **Corrected D2 (8aac792) failed narrowly, D130:**
  - Map 2 passes every check; map 1 fails only outside trade, by one run; map 3 fails over 2,000, spacing and stability.
  - The cloth share jumps from 0.17 to 1.00 in 1350, against the record's two-century rise (about 0.05 in the 1350s to 0.85 in the 1540s).
  - Fix: mill capacity binds and grows with W. Calibrate the 1350s start (≤0.10) only; the path is the test (±0.15 of the record at 1400, 1450 and 1500).
  - Outside trade not revised (its denominator lacks the internal economy; decide later between share and exports per head if it binds again).
  - Map 3 diagnosis requested: population, pasture, the top 15, spacing side, stability.
- **Capacity-bound D2 (d786d8b) failed, D132:**
  - Spec error 14 (mine): the duties were counted twice. The wool port price is Lloyd's and Clark's English price, observed after the duties; D123 took them off again, halving wool's value and causing the coin drain (map 3 at 61k against 89k in 1600).
  - Fixed: no duties in the model; making cost 2.7 d from Munro (inside the corrected bracket 2.4–3.2, so the cloth share is a real test); home wool elasticity 0.3; capacity the largest value giving a start of 0.10 or less.
  - Over-2,000 band to be decided on sources if map 3 still fails with population restored (urban share requested).
- **D160:**
  - Spacing trace: no traceable series effect (resample). Paired re-gate on seeds 41–80 for D144 and the series; map 3 spacing judged on 80 seeds against D144's own count minus 4.
  - D3 exploration: income per head 1.02 in 1500 against labourers' 2.3; cultivation stops at need, so survivors get no more land. D3 waits on a land-per-worker note (margin rule, holdings, output per head, growth, Hatcher, the 1600 overshoot).
  - Map 3 spacing is the next robustness target after D3.
- **D158:**
  - The wool series alone fails map 3 spacing (25/40; it slid 34 → 30 → 25). A trace of map 3's failing runs comes first.
  - D3 decided: Dyer is 3 yd an adult; Houthakker elasticities (clothing 1.0, stone 0.8, household iron 1.0; plough iron not income-elastic); salt fixed; income per head against the region's 1300–39 mean; checks reported, not fitted.
  - Build D3 off and explore while the trace stands.
- **D156 (replaces D155):** grades dropped (their reason went with putting-out; the good-share prior capped the share at 0.4). Wool series alone: exploration, then the gate. Then the D3 note.
- **D155 (replaced):** grades and the wool series gated on sources (option a); the post-1500 cloth share (0.37–0.41 against the record's 0.55–0.85) is accepted as a known miss, because capacity is tied to W. Then the D3 note.
- **D153 (dcac01e), D154:**
  - Clark's wool falls against wheat (0.49 by 1500); Munro's cloth rises (×1.37–1.66). Spec error 16: I asserted a series' direction unseen.
  - Putting-out set aside (explored, not adopted): on sourced prices it takes the whole clip from 1490; the residual raw trade was institutional (the Staple).
  - Now: grades and the wool series gated with capacity tied to W (exploration first, per the implementer's new report-before-run rule). Then D3 (wealth), note first.
- **Implementer rule (Divish):** when an instruction looks likely to fail, the implementer reports before the run and gates only on confirmation.
- **Grades and putting-out (78edbaf) failed, D152:**
  - Grades work to 1480 (share 0.55); from 1490 the later cloth price outbids every grade (share 1.00, boom back).
  - Spec error 15 (mine): cloth followed its price series while wool was frozen after 1400.
  - Fix: wool on Clark's series for the whole run; re-gate the three together with D150's pre-registrations.
- **D149 (a399bf1), D150:**
  - The attribution shows the boom flattens map 1, not the payment split.
  - Decided: wool grades within the clip (Munro 100:70:49; mean kept at Lloyd's); grade shares from a source or equal thirds as a prior, never fitted to the path; one cloth netback for all grades.
  - Order: grades alone (reported), then grades and putting-out gated together.
  - Pre-registered: share below 0.95 always; 0.35–0.65 and 0.55–0.85; map 1 rank-size 30/40 or better; coin no more than 1.5× D144.
- **Putting-out (c58d5c2) failed, D148:**
  - Rank-size on map 1 is 9/40. Early growth matches the record (×4 against ×3, 1360–90), but the clip binds from 1390 and the share is 1.00: one wool at one price, the all-or-nothing risk.
  - Next: an attribution at 20 seeds (payment split against cloth boom) and a note comparing wool grades (Lloyd) with bounded cloth market access.
- **D146:** exploration runs at 20 seeds or fewer; the gate stays at 40.
- **D142 done (fad4223), D145:**
  - Graphics merged with the golden unchanged; the later cloth price is the default (golden `fc16de69c1855b30`, 22/35/20); map 3 spacing at the floor (30/40).
  - Putting-out accepted: a capacity stock growing on margin, W's ε, κ_c calibrated at the start, s 25 lb as a ceiling, f from Munro or 0.3.
  - Pre-registered: share 0.35–0.65 (1440–60) and 0.55–0.85 (1490–1510), below 0.95 always, map 3 spacing no worse.
- **D142 (24 October):** resume after the graphics session's overhaul merges (the golden must stay `2e6856056e73a1c6`), then the D141 order.
- **PAUSED at D141 (9c84611):** payment fix default, golden `2e6856056e73a1c6`, exports/output 0.066–0.078. On resuming: (1) output counts milled wool twice, so count cloth by value added; (2) gate the later cloth price; (3) putting-out note.
- **PAUSE ordered by Divish (23 October):** development pauses after the implementer's running D139 test finishes. Record its result, then stop; no putting-out note until resumed.
- **D137 result (134fa79), D139:**
  - The payment fix passes alone: adopt it, gated.
  - The later cloth price breaks outside trade (medians 0.50–0.53) but moves the cloth share to the record's level about 40 years late.
  - Outside trade re-founded: exports over total output, 1–35%, England about 6–7% (third change; it now has a referent).
  - Then the later price is gated; then a putting-out design note.
- **D2 default (c7f9312), D137:**
  - Gate passed (25/36/28); golden `5e773d6f60251a23`.
  - Trace: capacity follows mill W, which barely answers the margin. Mills paid sellers about nothing (a defect: they paid from the purse before the proceeds).
  - D137: mills pay the clearing price from the proceeds; Munro's later cloth price, deflated by Clark's wheat; gate.
  - Pre-registered: the cloth share stays below the record. If it does, the missing mechanism is putting-out labour entering cloth for its returns, as a design note first.
- **RESUMED by Divish, 23 October (D135):** over-2,000 made reported (floor 5 is several times England's per-head rate of about one town per 80–100k in 1377; the model's urban share is 0.33–0.37 against England's 10%; the hierarchy is gated by rank-size, largest share and over 5,000). D1b and D2 become the default after a formal re-validate. Then the cloth pace trace (capacity, mill W growth, netbacks, cloth price after 1450).
- **PAUSED by Divish after the D132 report (D134).** The no-duty D2 (34de135) fails only map 3's over 2,000 (22/40); cloth runs at half the record's pace after 1420. Open on resuming: the over-2,000 band (map 3's spread) and the cloth pace. The model is at D1.
- **For when development resumes (D118):** Divish's demand proposal (`docs/proposals/demand.md`) accepted: demand as replacement of stocks plus growth; chains only where a separate trade sits between (cloth first and alone, at fulling-mill towns); wealth raises stocks; building stock from W; milling toll left out. Order on resuming: robustness re-tune, then D0–D3.
- **DEVELOPMENT STOPS (Divish, 22 October 2026; D117):** after Kent geology. The robustness re-tune, the cloth note and Tudor growth are not done, and are kept as the next steps if development resumes.
- **v7 RELEASED (1a94a88):** README rewritten for the trade economy; parameters regenerated; watching panel with seven phases; prices, purses and stores in the inspector, plus a grain-price layer (red in 1316); screenshots in `docs/screenshots/v7/`; golden `65aee272c30de1f9`, 45.9 ms/year wasm. Kent: largest near Gravesend 14/20, inland spacing 13.2 km (real 10.6). Kent geology next.
- **D116 (after the v7 release):** 1) Kent geology layer (BGS 625k: chalk and marsh lower arable yield; pre-registered pasture on the Downs and Marsh above the Weald and lowland); 2) robustness re-tune (most runs passing every check, gates kept); 3) cloth design note (fulling at mills, cloth replacing wool in the 1400s, the Weald a cloth country); 4) Tudor growth noted.
- **v6 markets decided (D70, number reserved by me):** Divish's proposal (`docs/proposals/markets.md`, through a third session, `session_01MrMcFJCCYku1zTBbBkDVfp`) accepted in staged form after the v5 release: entropy-regularised spatial price equilibrium in grain units; W kept as a market premium; Clark checks reported at S0 and gated at S2; S0 (shadow prices, no feedback) only, then stop. S4–S5 (goods in use, coin and mayors) are not scheduled. "Never planned" now names wages, labour markets, floating money and agents.
- **Roadmap after v2 decided** (`docs/foundation.md`, "Plan after v2"): v2.1 Kent resources as data → v3 (shadow in km first, then traffic roads, bridges at busy crossings, mills) → v4 shocks (plague, famine years, river shift, lords planting towns; no borders or tolls) → v5 decided later; outside world folded into v5. Divish left these calls to me.
- `docs/foundation.md` matches the code as of D32: tuned defaults stated as rounded values with `docs/parameters.md` authoritative; food is consumed at the market it reaches (no redistribution to senders); crossings are free at fords only.

## What the Kent test taught, and the rules that came out of it

Ten of my specification choices have been wrong, all caught by the real-terrain test. The pattern is worth remembering because it will recur.

1. **Attraction was the wrong quantity.** I had derived market pull from "people beyond what the fields feed", so a place could only become a market by running out of land. The published model has market size as its own state. Rule: when simplifying a published model, keep its state variables; simplify parameters, not structure.
2. **Growth had only one currency.** Food was the only thing that fed growth, so a gateway earned nothing from the trade it handled, and London could not form. Handling income fixed it. Rule: a break-of-bulk mechanism needs an income term, not just a cost term.
3. **City-scale weights were applied to hamlets.** Bosker & Buringh's transport weights are for cities over 5,000. Villages followed farmland and water. Rule: match each mechanism to the scale the evidence was measured at.
4. **The water rule was a river rule.** "Water within 1 km" with a 60 km² stream threshold forbade founding on most of the Weald and the Downs, which are full of springs and brooks. Rule: a filter must be checked against the share of land it admits before it is trusted.

5. **The Roman roads expectation rested on a mechanism the model lacked.** Rochester and Canterbury were expected from road–river crossings, but crossings were never bridges. Rule: a pre-registration names the mechanism its expectation depends on, and the implementer confirms the code has it before running.

6. **The failure rule deleted towns.** "Unfed for 20 years, removed" was written for hamlets and applied to every size, and it read market lapse (39% of markets survived) as places vanishing. It contradicted the brief's own "towns outlive their reason". Before the probe found it, I withdrew the Rochester expectation on reasoning that turned out wrong. Rules: when a pre-registered test fails, measure the mechanism directly before choosing a fix or withdrawing the expectation; and check every removal or cut-off rule at each scale it acts on.

7. **Goods were sold whether wanted or not.** I specified goods trade as the mirror of the grain step (producer as origin), so salterns earned at their cap on salt nobody needed and became the largest towns. Goods follow the shopping model (buyer as origin, demand fixed). Rule: for each flow, ask who is obliged to trade, the seller or the buyer, and put the origin there.

8. **I scheduled local weather before the buffers that absorb it,** and then before checking the growth rule itself. Zero-mean harvest noise, with losses asymmetric (shedding in bad years, capped growth in good ones), cut map 1's population at year 200 to a third and failed three checks. Four gates later the deeper cause was the growth rule: capped gains and uncapped losses turn any variance into a drain. Rules: add a source of variance only after the mechanisms that absorb it exist, and first test every response it passes through for asymmetry.

9. **I forbade the right quantity because it looked like an old fault.** M2 counted only grain sold, because grain arriving resembled P3. But arriving grain is food supply, and food supply is what makes a town grow. P3's fault was permanent waste, not counting arrivals. Rule: before ruling a quantity out, say what it means economically, and test the fault itself (here: waste ≤ 10%) instead of banning the quantity.

10. **I asked hunger to do competition's job.** Three M4 builds tried to keep spacing with ever cleverer hunger averages. Spacing was being propped up by famine culling all along. Rule: when a change breaks a check, find the mechanism that is supposed to produce that check's result, and fix it there.

11. **I pre-registered a gain inside the noise.** R expected 12 or more runs passing every check on every map, up from 8/13/8, from levers tuned at every milestone. At 20 seeds those counts move ±2 by the draw alone, so the best of 24 settings looked better by chance and lost on fresh seeds. Earlier tunings (D111, D113) had answered one-seed shortfalls. Rules: before pre-registering a gain or setting a gate, compute the run-to-run noise at the sample size; pick among many settings only with a confirmation on fresh seeds; and do not tune to answer a shortfall smaller than the noise.

12. **I predicted from the textbook mechanism without asking which prices the model leaves free.** D121 expected Hume's falling price level. But the region takes its prices from outside (wool, grain at port parity, iron and salt at the port), so it adjusts through coin and quantities: the monetary approach (McCloskey & Zecher 1976). The same slip, a cap on price read as a cap on size, may explain the map 1 specialists. Rule: when pre-registering, say which quantity the mechanism fixes and which it leaves free.

13. **I let a chain into the home.** D123 accepted home cloth made at mill towns and bought by everyone, against the owner's own rule (D118.2: a chain only where a separate trade in another place sits between). Everyday cloth was spun and woven at home and in the village, so the build moved a quarter of everyone's food value to a hundred small places on streams, and spacing, rank-size and recovery broke. This is rule 7 again: for each flow, ask who actually traded with whom, and check each new flow against the rules already decided.

14. **I taxed a price that had already been taxed.** D123 applied the wool subsidy and custom to the port's wool price, but that price is Lloyd's and Clark's English price, paid to growers after the duties. Wool's value was halved, the clip was pushed onto households, coin drained, and map 3 lost a third of its people. I also read the gap it made against Munro as a missing mechanism. Rule: before applying a levy, cost or margin to a price series, ask on which side of it the series was observed.

15. **I moved one price and froze its rival.** D137 put cloth on Munro's later price series, but wool stayed flat after Clark's fall ended in 1400. From 1490 cloth outbid every grade of wool, and the boom came back. Rule: when one price follows its historical series, every price it competes with follows its own series over the same years.

16. **I asserted a series I had not seen.** D152 said English wool prices rose in the sixteenth century. Against wheat, Clark's series falls (0.49 by 1500, 0.35 by 1590). The implementer caught it before a run. Rule: before a decision rests on what a series does, have the series in hand.

Judgement rules that now stand:

- **Pass rates, not single runs.** Each check passes in at least 75 % of 20 seeds on every map. A single seed on a range edge is noise.
- **A check the model cannot meet for structural reasons gets reconsidered, not tuned toward.** The corridor check went from gated to reported because, with a populated middle tier, which river gets the big city is contingent (Evans & Rivers 2017). The per-seed siting check carries the transport claim.
- **Define a gate on what it is for, not on a label.** A check on a category is only as good as the rule that assigns the category.
- **Judge placement where the inputs have signal; judge count and spacing where they do not.** Score the inputs directly against the ground truth before blaming the model.
- **Second-nature inputs (roads, initial towns) are data in `Terrain`, not model changes.** They go in through the importer.
- **Keep the three-crate boundary.** Pure `core` with no I/O; `Snapshot` is the only thing the viewer and CLI read; systems write only their own outputs; one `ViewState`, one `Msg` enum in the UI.
- **Never planned:** individual agents, wages and labour markets, hosted CI, pull requests. (Market prices admitted by D70; money as a stock by Divish, D96.)

## How to respond to an implementer report

The implementer's reports are careful and honest; they include negative results and corrections to their own earlier claims. Treat them that way.

1. Read the numbers before the recommendation. Check the baseline is there.
2. Say whether the result is a tuning problem, a spec error, or a limit of the inputs. Those get different answers.
3. If it is a spec error, say so plainly, including when it was mine, and give the corrected rule with its research footing.
4. Approve at most one round of change at a time, with the tables it must report and the condition for stopping. "Run it once, then stop and report" is the standing shape.
5. Keep `docs/foundation.md` matching the code. After each accepted change, rewrite the affected sections together rather than patching.

## Reference points

- Brief: `docs/foundation.md` (sections: expected behaviours, research basis, model specification, validation targets, architecture and UI architecture, working agreement, future versions).
- History: `docs/decisions.md`, `docs/milestones.md`, `docs/real-terrain.md`, `docs/real-terrain-experiments.md`.
- Reproduce the Kent test: `just real`.
- Research the brief rests on: Bosker & Buringh 2017 (city seeds); Rihll & Wilson 1987 and Evans & Rivers 2017 (urban dynamics and contingency); Barjamovic et al. 2019 (gravity on Bronze Age trade); Bleakley & Lin 2012 (portage and persistence); Dalgaard et al. 2022 (Roman roads); ORBIS (transport cost ratios); von Thünen; Christaller; the English Gazetteer of Markets and Fairs and Bracton's 6⅔-mile rule.

## Tone

Plain, specific, no cheerleading. Short replies to short questions. Decisions stated as decisions, with the reason in one or two sentences. Credit the implementer's findings explicitly, and own the designer's mistakes the same way.

## Sessions

Messages go by the channel rules in `CLAUDE.md`.
- Designer: `session_01FV7PafZcE8MXsZnCyjBuYd`
- Implementer: `session_014BA7MX2NmHLKnGyB3DXHv5`

## Keeping this file current

This file lives in the repo at `docs/designer-context.md` and is the designer's working memory between sessions. Update it in the same commit as any designer decision, brief revision or judgement on an implementer report: the state of the project, the agreed next steps, and any new rule. Rewrite stale sections rather than appending.

Last updated: 2026-10-24 — D160: paired re-gate of the wool series; land-per-worker note before D3.
