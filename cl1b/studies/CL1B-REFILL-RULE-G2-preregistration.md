# CL1B-REFILL-RULE-G2 · actor-paid executable decision-parameter revision

**2026-11-21 UTC · frozen BEFORE first original Rust native G2 run.** Parent [G1 184-world full-native audit](../results/CL1B-REFILL-GATE-G1-source-pinned-184-original-native-paid-conditional-refill-audit.md), [CI #44915739894](https://github.com/iFrankStudio/cioc/actions/runs/44915739894), tested exact SHA `51c64a3c1b0fa3fb527c473581746f74bf22774d`. G1 proved ordinary paid conditional execution, but its immutable Branch edges were preselected by researchers and conditional policy lost to never-refill.

## Question and nonclaims

Can the original paid CIL actor **actually edit a Const parameter that a later live decision consumes**, all in the original **16 data cells / 180 executable nodes / 300 occupied units / 6000 private compute** regime, with no source edits performed by the host? Does this authored rule save native compute against two identically priced fixed Branch controls? An actor-paid change of a researcher-authored parameter is **not** evidence of invented algorithms, useful autonomous learning, B3/B4 acceptance or cyberlife.

## Frozen source construction, exactly 180 nodes

Start from exact original 180-node F6 in R1. Changes ONLY at PC15, PC25..28 and PC171..179:

- PC15 wrong-third-probe branch formerly 25→28 (write cell21=0 and halt) now goes to **original PC61 Halt** with no success claim: fail closed, reused original former failure-handler slots become live monitor subroutine.
- PC171 = Const r2,39→172; PC172 = Write [r2]=r0→173 (r0=0 after PC170 threshold); PC173 = Const r2,theta→174 (initial theta=0); PC174 = Sub r0=r3-r2→175; PC175 = Branch test=r0 (the one differing G2 node).
- Branch variants: `G2_changed_allow` nonzero→176, zero→179; `G2_changed_deny` nonzero→179, zero→176; `G2_always` both→176; `G2_never` both→179.
- Allow path PC176=Const r0,0→177; PC177=Const r2,38→178; PC178=Write [r2]=r0→179, restores quota with an **ordinary paid CIL write**. Skip path bypasses these instructions. No researcher native write suppression.
- Common path PC179=Const r1,133→25; PC25=Const r0,62→26; PC26=PatchNext node[r1].next=r0→27 (paid monitoring disarm); PC27=Const r1,173→28; PC28=PatchConst node[r1].value=r3→125 (paid **self-modification** of live future PC173 Const); unchanged PC125=Const r2,32→126 and PC126=Read r1=[r2]→62 recycle original stage-read path.
- Code length remains 180 for each arm. The **four G2 arms differ at PC175 only**; all must execute the same budgeted patch instruction even when the branch is always/never. Original F6 is a freeze source, NOT a G2 fifth arm.

The updated parameter is the last task guess r3, which may be a poor proxy for future regime shifts. The candidate and every alternate are researcher-authored. Genuine online paid source modification is the only new capability under test.

## Pre-native experimental matrix

Freeze literal **45 NEW 24-goal vectors**, seed32 `0xC11B6B92` xorshift32, five task families × start0..2 × stride0..2. All frozen goals must differ from prior R1 189 task vectors and prior G1 45 task vectors (not necessarily independent of their generator family). Each vector loads four independent original single-PID1 Rust worlds, totaling 180 truthful. Add four deception controls: `inverted` and `forced0` each crossed with `G2_changed_allow` and `G2_never` = **184 worlds**. No world inherits cross-world memory. Fixed family order and equal weights; do not choose a favorable posthoc mixture.

Primary: successful completion of **all true 24 task stages** in each truthful arm first, then all actual charged CIL steps compared pairwise `cost(never)-cost(changed_allow)` and `cost(always)-cost(changed_allow)`, family-wise and total. Positive means conditional is cheaper. Report alternate changed_deny separately with no retrospective favorable policy relabeling. No invented extra probe costs.

Secondary: actual paid PC28 PatchConst attempts, changed PC173 values, later paid PC173 use of updated values, PC175 native decisions, paid PC178 quota refills, paid PC26 monitor disarm, source occupancy, private credits, real task probes, and false success under inverted feedback.

**Falsifiers:** (1) fail closed if original native strict VM limits or altered-source whitelist violated; (2) in at least one same-PID run PC28 must change PC173 and later PC173 must execute that changed value; (3) branch result must be completely explained by the in-process paid CIL instruction and mutable source, including independent Python exact row replay; (4) forced0 must halt without fabricated success and inverted feedback not be mistaken for real service; (5) full 103-target native test campaign and 63 observers + independent trace verification must be checked by exact source SHA before a VERIFIED claim. An actor executing an externally specified mutator is Tier-1 engineering, not autonomous discovery or CL1B B3/B4.
