# G3Q-C1 identifiability gate — bounded abstract negative result (2026-11-19)

**Status: local Node.js abstract calculation only; NOT original Rust PID1 evidence, native CI, or a completed C1 experiment.** This note supplements, and does not amend retroactively, the frozen [C1 preregistration](../studies/B56-F1d1k-G3Q-C1-paid-certified-safe-rule-deployment-under-task-shift-preregistration.md).

## New falsification result

The finite G3Q grammar uses 3 meta words from `{0,1,2,3}`; a word `3` resets the accumulator and the second word is transformed by the trial index. The 64 descriptors produce exactly 24 certified descriptors whose 3 trial outputs are permutations of candidate modes A/B/C. Enumerating 24 certified rules crossed with each of the three reachable targets yields **72** rule-target pairs. When the service is successful if a target is attempted within a fixed probe budget, every certified rule has exactly:

| Probe limit | Successes per rule on 3 targets | Successes over 72 cases |
| --- | ---: | ---: |
| first 1 | 1 | 24 |
| first 2 | 2 | 48 |
| first 3 | 3 | 72 |

The first mode histogram among certified descriptors is **A:8, B:8, C:8**. So a full-coverage safe-bank certificate does **not** identify useful task-specific learning, even when limiting the number of probes. The previously computed full-budget C1 comparison (certified fallback = 192/256 and privileged static = 192/256) is structurally expected, not evidence of adaptive superiority.

Explicit conflicting first-probe witnesses: original C0 retained bank `[1,3,2]` yields modes `[2,0,1]`, while safe descriptor `[2,2,2]` yields `[0,2,1]`. For target A, the alternate is first-probe correct and C0 bank is not; for target C, the reverse holds. Neither sequence dominates the other. The x+4 unreachable task is **still unreachable** in both.

## Native C1 consequence: predeclare the *identification* criterion

1. **Provenance/feasibility gate** remains mandatory: same original PID1 pays to generate, classify, retain, then actually Read and deploy safe descriptor bytes into active rule execution, followed by original task-code opcode fetch and service Write11. An external researcher's constant, host injection, and certificate-only Write11 do not qualify. Preserve 16/180/300 physics.
2. **Causal gate:** matched source-identical ON/OFF and charged monitor-only controls must show differences in actual source-paid behavior, not only an externally labeled success flag. For a certificate-to-fallback claim, intervene on the retained descriptor *only through a legitimate actor-paid experimental branch*, or compare an equally charged actor-owned saved-bank alternative; never silently mutate the data via host intervention. Trace the writer and exact triggering Read.
3. **Learning gate distinct from structural coverage:** preregister the hidden regime shift and probe budget, count task correctness and paid costs against fixed-privileged and admissible frozen baselines. In each regime, demonstrate selection change explained by earlier truthful feedback under matched source and budget, including targeted feedback deception/absence controls. A single all-target-safe permutation without feedback-conditioned ordering fails this gate.
4. **Failure gate:** account for all unreachable-target failures, rejection-only losses, and source-capacity infeasibility. If the 180-node program cannot realize certificate + task service + feedback + deployment, report infeasibility rather than outsource native certification to the host or increase caps.

A successful native C1 may still establish only **earned internal certification and deployment**, *not* adaptive rule invention. The existing 24/64 structural certificate result remains valid under its earlier narrow definition.

## Reproduction

`node cl1b/analysis/b56_f1d1k_g3q_c1_budget_identifiability.mjs`

The script exhaustively enumerates the finite grammar and asserts all figures. It is deliberately isolated from expensive CL-1B native CI, and asserts no native execution.
