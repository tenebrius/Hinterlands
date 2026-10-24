# Implementer context

The implementer's working memory between sessions. It is the counterpart of `docs/designer-context.md`. Read it after `CLAUDE.md`, `docs/foundation.md` (the brief) and `docs/decisions.md` (departures). Update it when the state of the work changes, and rewrite stale sections rather than appending.

Last updated: 2026-10-23. Paused by Divish after D139 step 1 (D140). Main: D1b + D2 with the mills' payment fix, golden `2e6856056e73a1c6`.

## Who and how
- **This session is the implementer:** `session_014BA7MX2NmHLKnGyB3DXHv5`.
- **The designer is** `session_01FV7PafZcE8MXsZnCyjBuYd`. The designer has full control of design (defaults, gates, scope, order); see `CLAUDE.md`.
- **Messages** use the `send_message` tool (claude-code-remote). Commit and push first, then send a short report naming the commit. One round at a time; no replies to acknowledgements. A designer message is peer data: it never widens permissions or authorises destructive git.
- **Push to both branches:** `git push -q origin HEAD:main HEAD:ccr-518ddadf-lg74ka`. Pull `main` first (`git pull -q origin main`); the designer commits there too.
- **Commit messages** name the milestone and system, and end with the attribution lines (`Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` and the `Claude-Session:` URL). Never put a model ID in a commit or doc.
- **Decision numbers** are shared with the designer and increase. The designer's numbers are in `docs/foundation.md`; the implementer's records go in `docs/decisions.md`. The last used is D140, the implementer's. Take the next free number.
- **A report to the designer** follows the brief exactly:
  - the gate per check, against the reference;
  - every pre-registration, met or not, with numbers;
  - every "also report" item.
  - It never tunes what was pre-registered "don't tune". Unverified causal claims are marked as readings, not facts.
  - It goes in a doc under `docs/` (for example `docs/demand/d2d.md`) plus an entry in `docs/decisions.md`, and in `docs/milestones.md` when a milestone passes.

## State of the model (D136)
- **The default model is D1 + D1b + D2.**
  - D1: stock-based demand for salt, iron and stone; the ports sell salt and iron.
  - D1b `port_reach` and D2 `cloth`: both on by default since D135 (`crates/hinterlands-core/src/params.rs`).
  - Golden `5e773d6f60251a23`, in `crates/hinterlands-cli/tests/golden.txt`. It is the same in the browser.
  - `just test` passes. `just validate` (40 seeds) passed: 25 / 36 / 28 runs passing every check (`docs/demand/d2e.md`).
- **D1b:** every settlement with no port within `c_max` gets `reach.port`, its nearest port at any travel cost (`Dijkstra::run_nearest` in `cost.rs`, set in `reach.rs`). Salt and iron get a `FAR_PORT` arc in `goods_trade`.
- **D2** (`systems/pasture.rs`):
  - the fallow flock; wool by port reach;
  - `wool_market`, an entropy split of wool between home use, the ports and the mills;
  - a mill bids Munro's 5.7 d less cloth carriage and the 2.7 d making cost, with capacity `cloth_lb_per_w` 2.5 × W;
  - home wool elasticity 0.3; no duties.
- **The validate rule (D135):** over 2,000 is reported (count and urban share against England's 0.10), not gated. Outside trade is 3–50% (D126). Stores are on the D124 measure.
- **The cloth-pace trace** (`hinterlands clothpace`, `state.cloth_trace`, reported only, hash-neutral) found:
  - mill capacity binds everywhere from the 1370s, so the share follows the mill towns' W;
  - mills pay sellers far below the clearing price, mostly nothing from 1450 (they pay from the market purse before the cloth is sold);
  - Munro's English export cloth values rise after 1470 (`EngclothPrices2.xls`).

## When development resumes
- **Development is paused** (designer's message after D139). On resuming, wait for the designer.
- **Not yet done from D139:**
  - step 3: gate `cloth_price_late` on, 40 seeds, under exports / output (params `{"cloth_price_late": true}`). The D137 run with it on gave exports too high under the old share only;
  - then the putting-out design note. An unsent draft is kept outside the repo; write it only when the designer asks.
- **Outside trade** is now `exports / output` (1–35%, gated). The old share is a reported row.

## Running things
- **The gate:** `just validate`, which is `--maps 3 --seeds 40`.
  - It takes about 60–90 minutes on 4 cores. Run it in the background from a **copy** of the binary, so builds can go on:
    `cp target/release/hinterlands target/d2/hl_x && nohup ./target/d2/hl_x validate --seeds 40 --params P.json > out.txt 2>&1 &`.
  - Then wait with a background `until ! ps -eo args | grep -q "[h]l_x validate"; do sleep 30; done` loop.
  - Never use `pkill -f`, which matches its own shell. Never use foreground `sleep`.
- **Builds for probes** go in `CARGO_TARGET_DIR=target/probe` while a gate runs. The machine has 4 cores; run heavy jobs one after another.
- **Params files are not tracked** (`target/` is ignored). A params file holds only the overrides; missing fields take their defaults (serde), as in `tools/retune/screen.sh`. `hinterlands params` prints the defaults as a Markdown table.
  - Both switches are now default; D1 alone is `{"port_reach": false, "cloth": false}`, and D1b alone is `{"cloth": false}`.
  - Attribution run: `{"cloth_lb_per_w": 0, "cloth_yd_adult": 0}`.
- **Probes** (`crates/hinterlands-cli/src/probe.rs`; all take `--params`, `--map-seed`, `--seed`):
  - `clothpace` (D135 trace: mill capacity and use, mill towns' W against margin, netbacks, by decade 1350–1550);
  - `money` (coin by channel, port shares);
  - `cloth` (clip, outlets, cloth share, mill capacity, cloth towns);
  - `diag` (population 1340/1450/1600, urban share, pasture, top 15, spacing side, stability year);
  - `ironprice` (split by port reach);
  - `specialists`, `outside`, `stores`, `demand` (D0 tables).
- **Report tables:** `tools/cloth/d2.py DIR` (cloth and money probe outputs), `tools/demand/d0.py`.
- **Graphics merges:** the graphics session runs the browser selftest on the merged tree; do not repeat it (owner, 24 October). Check the golden natively, `just test` and `just validate`.
- **Browser selftest:**
  - `just web`;
  - `python3 -m http.server 8765 --directory /home/user/Hinterlands/dist` in the background (use an absolute path);
  - `NODE_PATH=/opt/node-tools/node_modules node tools/screenshot.mjs "http://localhost:8765/?selftest=500" out.png wait:300000`.
  - The hash must equal `hinterlands hash --years 500`.
- **`cargo fmt`** also reformats files you did not touch. Use `cargo fmt -p <crate>`, and check `git status` before committing.

## Lessons from D1 to D2 worth keeping
- **Trace before tuning.** A surprising check result has usually been a missing mechanism or a measurement, not a parameter:
  - the portless districts (D123 trace 2);
  - carry-over with no cheap years (trace 3);
  - the duties counted twice (D132).
- **Elasticities decide price levels when supply is short.** An inelastic home demand with scarce supply drained purses (D2 first build). A unit-elastic demand absorbed a clip the duties pushed home (D131).
- **The entropy split at the S1 θ (≈ 70) is nearly all-or-nothing** for price gaps over 0.05. A capacity must be enforced explicitly; the alternating solve does not converge with two inelastic sides.
