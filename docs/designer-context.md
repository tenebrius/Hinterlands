# Designer context — Hinterlands

Read this at the start of a session when acting as the **designer** for Hinterlands. It restores the role, the state of the project and the judgement built up so far. The brief itself is `docs/foundation.md`; this file is what sits around it.

## The role

There are two roles on this project and they are kept separate.

- **The implementer** builds, tunes and validates. Works autonomously on `main`, no CI, no PRs. Records every milestone and every model change as a numbered decision (`docs/decisions.md`, D1…). Raises only the points listed in the working agreement.
- **The designer** (this role) owns the model. Decides what the simulation must do, judges validation and real-terrain results, corrects the specification when evidence shows it is wrong, and keeps scope from growing. Does not write production code. Writes the brief, the checks, and the reasoning behind each change.

The owner is Divish. He has had projects of this kind fail before from over-ambition, so the standing instruction is: **small, proven, gated**. Every mechanism comes from a recognised published model where one exists. Nothing from a later version is started before the current one passes its checks. When in doubt, cut.

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
- **Later options, not scheduled:** Kent resources as data (Wealden iron, ragstone, estuary salt); the outside world as a seller; the shadow in km; Kent inland spacing (6.2 km against 10.6).
- `docs/foundation.md` matches the code as of D32: tuned defaults stated as rounded values with `docs/parameters.md` authoritative; food is consumed at the market it reaches (no redistribution to senders); crossings are free at fords only.

## What the Kent test taught, and the rules that came out of it

Seven of my specification choices have been wrong, all caught by the real-terrain test. The pattern is worth remembering because it will recur.

1. **Attraction was the wrong quantity.** I had derived market pull from "people beyond what the fields feed", so a place could only become a market by running out of land. The published model has market size as its own state. Rule: when simplifying a published model, keep its state variables; simplify parameters, not structure.
2. **Growth had only one currency.** Food was the only thing that fed growth, so a gateway earned nothing from the trade it handled, and London could not form. Handling income fixed it. Rule: a break-of-bulk mechanism needs an income term, not just a cost term.
3. **City-scale weights were applied to hamlets.** Bosker & Buringh's transport weights are for cities over 5,000. Villages followed farmland and water. Rule: match each mechanism to the scale the evidence was measured at.
4. **The water rule was a river rule.** "Water within 1 km" with a 60 km² stream threshold forbade founding on most of the Weald and the Downs, which are full of springs and brooks. Rule: a filter must be checked against the share of land it admits before it is trusted.

5. **The Roman roads expectation rested on a mechanism the model lacked.** Rochester and Canterbury were expected from road–river crossings, but crossings were never bridges. Rule: a pre-registration names the mechanism its expectation depends on, and the implementer confirms the code has it before running.

6. **The failure rule deleted towns.** "Unfed for 20 years, removed" was written for hamlets and applied to every size, and it read market lapse (39% of markets survived) as places vanishing. It contradicted the brief's own "towns outlive their reason". Before the probe found it, I withdrew the Rochester expectation on reasoning that turned out wrong. Rules: when a pre-registered test fails, measure the mechanism directly before choosing a fix or withdrawing the expectation; and check every removal or cut-off rule at each scale it acts on.

7. **Goods were sold whether wanted or not.** I specified goods trade as the mirror of the grain step (producer as origin), so salterns earned at their cap on salt nobody needed and became the largest towns. Goods follow the shopping model (buyer as origin, demand fixed). Rule: for each flow, ask who is obliged to trade, the seller or the buyer, and put the origin there.

Judgement rules that now stand:

- **Pass rates, not single runs.** Each check passes in at least 75 % of 20 seeds on every map. A single seed on a range edge is noise.
- **A check the model cannot meet for structural reasons gets reconsidered, not tuned toward.** The corridor check went from gated to reported because, with a populated middle tier, which river gets the big city is contingent (Evans & Rivers 2017). The per-seed siting check carries the transport claim.
- **Define a gate on what it is for, not on a label.** A check on a category is only as good as the rule that assigns the category.
- **Judge placement where the inputs have signal; judge count and spacing where they do not.** Score the inputs directly against the ground truth before blaming the model.
- **Second-nature inputs (roads, initial towns) are data in `Terrain`, not model changes.** They go in through the importer.
- **Keep the three-crate boundary.** Pure `core` with no I/O; `Snapshot` is the only thing the viewer and CLI read; systems write only their own outputs; one `ViewState`, one `Msg` enum in the UI.
- **Never planned:** individual agents, a full price economy, hosted CI, pull requests.

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

Last updated: 2026-10-21 — v2 gate re-run failed on a label; kind now follows the economy (D43).
