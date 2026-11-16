# CL-1B B30 — prospective held-out packet threshold falsification of B29's sibling advantage

**Registered before modifying B29 code or computing B30 outcomes.** B30 is a *finite deterministic falsification study*. It does not imply spontaneous selection, biological fitness, generalized utility, or that reproduction is required by CL-1B.

## Hypothesis and scientific reason

B29 showed that an authored, internally executing CIL mutation-rule flip can produce two different full-program siblings from the **same parent**. Its third environmental packet happened to have exactly156 resource credits: enough to fund a `Draw155` child, but not enough to fund its parent `Draw156` or higher-request sibling `Draw157`. Hence FLIP made a third birth whereas same-cost NO_FLIP did not. This benefit could be a **narrow, preselected threshold effect**, not a robust evolutionary advantage.

Freeze the **exact same 23-node native B29 founder**, default request156, initial signed δ=+1, initial2 local credits, same generic restricted CL-0 VM and resource accounting. One host-loaded ancestor; no host `SetCredits`, code patch, offspring allocation or winner choice. For every world both source variants must receive the **same first two neutral packets `[157,157]`**, giving an identical first two-birth opportunity. Vary **only the third common-pool packet** `x ∈ {155,156,157,158}`.

Each choice of x is one held-out environmental condition, *not* a separate reselected code variant. Test two authored conditions at each x:
- **FLIP:** normal charged PC22 `PatchConst` alternates mutation δ after each native birth;
- **NO_FLIP:** PC22 is a same-cost ordinary `Const` and does not change source δ.

**Eight frozen worlds**, all native charges, residual pool credits, process PCs and actual `GraphLaunched` parent-child edges retained, plus source mutation traces. Do not silently select favorable x, and do not claim a p-value from these four discrete values.

## Preregistered mechanistic predictions (must be falsifiable)

- All runs first produce `(1→2,1→3)` under the same first two 157-credit packets. In FLIP, process3 is the child with request155, while NO_FLIP process3 has request157.
- Third packet **155**: neither process can afford the per-attempt Draw minimum156, so **both variants remain at two births** with 155 unused pool credits. No new mutation event should be invoked.
- Third packet **156**: **FLIP three births**, attributed to process3 as parent `3→4`; **NO_FLIP two births**, leaving156 pool credits. The previously observed B29 improvement is reproduced.
- Third packet **157**: both variants can fund the original parent1's `Draw156` (minimum157). Native deterministic scheduler may give parent1 priority, yielding a third parent1 birth in **both arms**, eliminating FLIP's **birth-count advantage**. Must record whether the third birth is actually `1→4` and not infer ancestry from ID alone.
- Third packet **158**: likewise both policies are eligible; same-source/native scheduler order may decide which competitor executes. Record exact ancestry, resource residuals and any deviation from the predicted three births.

## Native and independent observer verification

Extend the existing **single B29 Rust integration target** with this 8-world comparison; avoid duplicating its full test fixture. A source-parsed B30 observer must call the same **independently implemented B29 CIL interpreter** against the four frozen third-packet values; both independent and native must record the complete eight-case denominator.

No new VM, no extra resource minting, no learning/fitness/reproduction special primitive. Verify per-attempt local+pool energy decreases by exactly1, packet increases by exactly x, and the first two windows are matched before checking the third. A failed preregistered lineage/count assertion must remain visible with its CI run, and any correction be labeled post hoc.

**Acceptance:** B30 is a negative robustness test showing how **threshold availability and deterministic scheduling** can erase the apparent advantage of a researcher-authored mutation-rule inversion. It cannot on its own establish endogenous evolution or a continuously adapting individual; it specifically warns against equating one favorable environmental timeline with open-ended adaptation. The optional reproduction avenue must not replace CL-1B's central single-individual goal.
