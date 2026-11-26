# C36-R3 — hidden change and finite credit ledger

Parent 91d89b5e95fcf840c3132fe3ab0cbb22d001ffd0; corrected implementation 9ef3f77c46026f0e4f3ad280df243199c09ee880. 384 paired schedules x 7 arms = 2688 model worlds per run, 73728 offered requests per arm. Six new seeds, four regimes, initial credit 100/300, correct-production reward 40/65, storage cap84/96, install tariff1/4. No epoch boundary notification, no future horizon, no coefficient exposure except two paid basis samples (6 credits each). Only one-bit production acceptance and correct-output reward are available. Abstract ledger charges policy, observation, data, arithmetic, synthesis, install/retire, dispatch, publication, feedback; credit never negative, accounting exact. Exhaustion stops world; no other resource source. Equal reserved base storage 78; conservative max 11 additional graph nodes. Probe-all checks every request; feedback learns after failure; periodic4 adds checks after four requests; compiled variants install after observed coefficient change subject to admission; no-feedback suppresses rejection-driven relearning; compile-off retains paid graph and storage but interprets masks. Corrected first implementation's admission estimate to include synthesis charge (12+11*(tariff+2)+1); asserted actual install <= bound. Reran all worlds; results below supersede initial run.

| arm | correct | wrong | abstain | survived | exhausted | end credits | installs |
|---|---:|---:|---:|---:|---:|---:|---:|
|probe_all|17512|0|16|76|308|84276|0|
|feedback|17624|1292|132|80|304|291168|0|
|periodic4|17856|932|28|84|300|251400|0|
|feedback_compiled|18343|1223|43|89|295|344824|542|
|periodic4_compiled|18197|869|83|88|296|286103|562|
|no_feedback|3228|2028|112|0|384|52|0|
|compile_off|15100|1037|61|69|315|243549|331|

Survival means no exhaustion before completing 192 offered requests and final credit >=16; it is not indefinite viability. Credit reward only for correct output, no additional wrong-output fine. Wrong outputs can be accepted coincidentally if the current input does not distinguish coefficients. Probe-all zero wrong but many worlds exhaust. Feedback vs no-feedback survival 80 vs 0; compilation vs compile-off survival 89 vs 69, identical initial policy/graph charges but later state diverges with credit. Not a matched execution trace ablation. At cap84 admission always denied by conservative bound, even if some actual shorter graphs fit; overhead without gain. At cap96/reward40, feedback compiled survives 14/96 vs static feedback 0/96. At cap96/reward65, feedback compiled survives 37/96 vs static feedback 40/96, but higher aggregate remaining credit (190720 vs145556). Correctness, coverage and credit must all be reported; lower total paid can mean early failure.

Fast and switch (fast first 96 requests) have zero survivors in all arms; no evidence of adaptation to later slow phase because all die before that transition. Slow regime feedback compiled gives 11154 correct, 298 wrong, 56/96 survivors and 233216 end credits vs static feedback 10308 correct,276 wrong,48/96 survivors,195068 end credits. Mixed periodic static yields 7104 correct and 36/96 survivors vs feedback compiled 6401 and 33/96. No universal best. Observations reveal coefficients exactly, fixed family and authored policies. Host JS abstract operation model, not guest CIL, not native VM/CI, no B3/B4 promotion. Next: use matched capacity with less conservative paid admission, separate boot/setup liquidity from long-run sustainability, add recoverable switch schedules and verification-before-publication tradeoff. Audit native CIL feasibility before implementation; no claim of autonomous general environment discovery.

Reproduce: `node cl1b/analysis/cl1b_c36_r3_hidden_change_credit_ledger.mjs`.
