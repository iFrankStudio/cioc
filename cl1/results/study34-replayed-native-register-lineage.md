# Study 34 — source-register reachability correction and observed native address shift

**Design:** retrospective diagnostic of the already-frozen Study33 cohort; 0 new independent worlds. **Do not** treat this as a successful CL-1 acceptance experiment.

## Independent verification

- Successful targeted CI: [GitHub Actions 43546238925](https://github.com/iFrankStudio/cioc/actions/runs/43546238925), SHA `0a855d2411aa90480e86d3ac7add8d10d08a5166`, stored artifact `cl1-study34-retrospective`.
- Full native Study33 replay and baseline independent auditor: 128 seed worlds, 9 paired arms, 1,152 runs, 27,648 exact epoch snapshots.
- Rust interpreter semantics: `Op::GraphReadTag.graph` and `.node` are **register indexes**; the runtime `GraphTagRead` event records resolved object ID, owner, node and tag. This **retracts** the earlier claim that the source graph2 read is structurally unreachable because the initial generator lacks the literal operand `graph: 2`.
- Verified revised independent Study34 event audit: 25/128 control worlds contain a successful *native process1* read of resolved graph2, owner2, node0. Its initial source tag is 11, mutated tag 19.
- **Native process1 successful DrawCompute:** 52/128 control worlds.
- **Source2 reads after first native process1 DrawCompute:** 4/128 worlds; reads before first DrawCompute: 22/128 worlds (overlap possible).
- **Native source-tainted address writes anywhere** (not necessarily the preregistered target): 3/128 control worlds; the corresponding graph2-mutant arm also has 3/128. Three control source-dependent writes were seen in seed IDs 2,600,005; 2,600,051; 2,600,112.
- Independent scan of the original full event log for these three paired seed-worlds found source-dependent writes to keys 12, 12 and 22 respectively in control, versus 20, 20 and 30 in `mutate-g2`, a +8 key shift matching the externally imposed tag change 11→19. Observed output values are equal in each pair. This is **descriptive post-result evidence**, not a preregistered favorable endpoint.
- Strict preregistered Study33 *target-address* source writes **remain 0/128** (required exact addresses 11 in control and 19 in mutant), and its strict paired joint-source-and-resource endpoint remains **0/128**.
- Native program code was unchanged between prefork and final snapshots, according to the targeted retrospective auditor. No endogenous repair or externally unsubsidized resource origin is shown.

## Mechanistic diagnosis

The earliest false premise was that literal graph register 1 cannot reference graph object 2. In reality the register can contain handle 2, and native source reads occur. The strict downstream bottleneck appears later: register-origin writes can take place at addresses other than the registered target (e.g., 12 and 22). These effects depend on externally prepared source graph/tag and researcher-endowed initial code/compute-pool conditions; therefore they do **not** constitute cyberlife.

The Study34 `source_address_write` gate is a **retrospective broader observation**; it is not a replacement for the strict endpoint. The independent CI audit records seed and arm gates, native event PC attribution, registered-address contrasts, and selected witnesses. The registered Study34 protocol also calls for full matched-epoch chronology; prospective replication must preserve such chronology, not rely on matched final counts.

## Decision

Pre-register a fresh independent seed cohort **before** writing/running its runner to test whether graph2 perturbation produces reproducible **paired, native source-provenance-dependent address shifts**. Require successful source reads, provenance-preserving def-use, equal output values, paired epoch, negative and sham controls, and unaltered initial non-goal-directed CIL grammar. Do not aim for guaranteed tag→address mapping, hardcoded repair, externally scheduled function recovery, or substitute this informational transfer for the CL-1 scientific acceptance threshold.

**CL-1 remains OPEN; no spontaneous self-maintaining cyberlife has been observed.**
