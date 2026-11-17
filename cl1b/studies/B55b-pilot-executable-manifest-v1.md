# B55b executable feasibility pilot v1 — pre-native frozen manifest

**FROZEN before any B55b native Rust source or first execution (2026-11-17). NO RESULTS.** This is a deliberately smaller **B55b feasibility pilot** under the scientific [B55b prospective goals](B55b-online-monitor-policy-revision-under-hidden-change-preregistration.md), **not the complete B55b E1/E2/E3+oracle+mutation-disabled experiment** and **not** the still-incomplete B55a 7×3×8 registration.

## Hard resources and actor invariants

- Original same PID1, process starts at B55a CIL program node92, no process replacement or spawned helper. Exactly **4,000 paid CIL instructions** per world, no refill, no pool draw.
- `World::with_strict_capacity(16,180,300)`; initial graph exactly **178 original B51c/B52 and B55a inherited nodes plus new edits of previously unused nodes**, two paid B51c clones later occupy nodes178 and179, no 181st node. If this geometry cannot fit, **fail/mark blocked, do not silently enlarge bounds**.
- A single identical native-CIL source for the five pilot arms. Original B51c 72 nodes and B52 20 nodes unchanged except the **researcher-authored B55b replacement of B52 node79 Branch** to new success/failure control nodes; the exact B55a 178-node literal is reused and only node155's selected NEVER value is changed to FOURTH before execution, plus a bounded new appended policy-rate controller in previously Halt-filled slots. All such founder changes are **authored design**. After initial creation, rate edits to node2.next and node76 Const must be **PID1-paid** PatchNext/PatchConst, never host edits.
- Training x probes `[5,-3]`, service x `[11,-7,0]`; original arithmetic A `x-2→-x+3→x-2`, B `-x-3→x+2→-x-3`; ecology E1 remains on original task for all stages and E2 shifts task after exactly24 service writes then returns after48. The task mode remains invisible to PID1.
- Seeds frozen `[101,211,307,401]`, paired across arms and ecologies, alternately original task A/B; input phase uses deterministic `seed_index mod3` just as B55a. 4 seeds × 5 arms × 2 ecologies = **40 native worlds**. Each offers **72 mandatory service output opportunities** (24 per stage), without excluding missed/terminated worlds. Output after first72 remains separately tagged in full-budget throughput.

## Arm definitions and what is NOT included

| Arm | Fixed external initial cell27 value | Semantics |
|---|---:|---|
| A ONLINE_RATE | 0 | Actor's authored four-probe selector chooses FOURTH; after a *sampled actual failure* it pays to patch watchdog route FOURTH→EVERY; after 3 actual sampled successes it pays to patch EVERY→FOURTH |
| B FIXED_EVERY | 77 | Source-identical seeded control: full-feedback every service output, no online rate edits |
| C BLIND_ONLINE | 0 | Same source/paid selection, but the external environment replaces all monitor-visible trial and service correctness bits with 1 (blind success) |
| D FIXED_NEVER | 30 | Source-identical seeded control: source writer2.next→30, no post-retention monitoring |
| E FIXED_FOURTH | 160 | Source-identical seeded control: source writer2.next→160, feedback check every fourth service write, no online rate edits |

The only A/B/C/D/E differences are the pre-start **ordinary external cell27 seeded-control value** and C's externally disclosed blind feedback; the host never writes PID source/PC/credits, hidden task label or selected source. A uses the exactly same monitor feedback bit20 truth as B/E, and the actor's success counter in ordinary cell29 (<=16 allocated cells). Native paid computation compares thresholds; the laboratory does not implement adaptation.

**No E3 forged-current-nonce arm, mutation-disabled G control or task-informed upper-bound oracle in this pilot.** Thus **full B55b mechanism/utility acceptance must remain NOT YET ESTABLISHED**, even if a 40-world pilot shows changes. Explicitly report this gap, rather than silently promoting a green CI run to the full prereg.

## Frozen route behavior and evidence

- Normal A begins with paid node2→FOURTH(160); after a hidden stage shift, the **first genuinely sampled failed feedback** moves node76's retained policy target to EVERY(77) via a paid PatchConst and node2.next to77 via a paid PatchNext, *before* the original B52 reopen-search path. Later, after a paid counter records exactly **3 sampled successes**, PID1 pays to reset counter29 to0, PatchConst node76 to160 and PatchNext node2.next to160. A later shift/return may repeat this cycle. The agent must observe those source changes in actual native event/graph state and **cannot depend on lab stage transition callbacks to edit graph**.
- Controls B/D/E keep the same original code cost and seeded route access; they must never edit the online rate as a hidden host implementation.
- The scoring tuples are **(C,W,M,P)** and preregistered historical `U=(C−2W−M)/max(1,P)`. Additionally report **total service outputs and total correct outputs over all4,000 paid instructions** (including all writes after first72) as a throughput metric; **the precise paid step at completion of the first72**, or a missing flag; paid monitor Reads; cost to first correct service write after a task change; number of rate changes including before/after edges; trial and restart costs.
- **Hard falsifiers:** no true PID-paid feedback Read after a hidden change; A lacks both observed post-shift rate transitions; 2→77 or77→160 patches only happen in Rust host; no actual recovery under task change; mismatch of original source SHA; any paid-step/event divergence; capacity change/compute subsidy.
- **Mechanism-positive pilot** requires an actor-origin branch/patch sequence causing nontrivial FOURTH→EVERY and EVERY→FOURTH transitions in at least each hidden-transition family and input phase, native recovery after change, and full independent instruction replay. **Pilot utility-positive** requires better measured 4,000-step correct-output throughput than at least one fixed control in an ecology; comparison against both B and E with noninferiority E2 must be reported **without moving the goalpost**. These remain engineering-mechanism criteria, **NOT B3/B4**.
- The 4,000-step cap means the after72-service outputs are continuing service and **not** new future regime changes; objective truth remains the frozen third-stage task.

## Pre-test controls

Implement native test + separate source-parsing independent JS interpreter, register target/observer in the focused workflow, and emit this exact UTF-8 manifest's SHA-256 in native trace and independent replay. The manifest hash is **computed from bytes**, not hand-invented. The JavaScript observer may only use source instructions and external protocol schedule to derive output; it must not use native outputs as state inputs. Preserve failed runs and exact source revisions. The original B55a/B54 tests and analyses must still pass.

**STATUS: pre-native frozen feasibility protocol; no B55b native result, full B55b not completed, B3/B4/cyberlife NOT MET.**
