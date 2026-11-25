# C35-R1: paid independent verification

Parent 20d7d43b0f16b4e9edb7d01351b7e3efe4e230a8; implementation 714e26a6a2b7c36aeafc776b68fb071f9d196b4f. Guest-owned countdown schedules paid recomputation of the first product with the trusted original parent. Relation check plus independent comparison detects correlated silent errors on audit requests; mismatch triggers existing paid structural maintenance. Counter, routing, checks, repair and retry all charged. No host correctness oracle. Same 12 designed schedules, eight arms, 96 model worlds, 720 requests per arm.

| arm | paid | raw savings | wrong pairs | audits | repairs |
|---|---:|---:|---:|---:|---:|
|direct|57330|0|0|0|0|
|reactive|46891|10439|114|0|18|
|audit1|80950|-23620|0|724|20|
|audit2|67941|-10611|4|362|20|
|audit4|60571|-3241|8|182|20|
|audit8|57271|59|8|92|20|
|audit16|55577|1753|24|44|20|
|audit4_off|91880|-34550|378|264|0|

Matched repair-suppression prefix and zero actual repairs verified. Every audit request correct in nonsuppressed arms within this scoped fault model. Between-audit publication can be wrong; bounded retry is not safe rejection. Raw savings are not correctness-adjusted. Every-request recomputation eliminates scoped errors but destroys savings; no arm achieves both zero errors and positive savings. Parent trusted/unfaulted, schedules and verifier authored. No native run, replenishment, or B3/B4 promotion. Next test cheaper paid independent structural witnesses against held-out and nonrepairable faults, then reciprocal resource replenishment if justified.

Reproduce: `node cl1b/analysis/cl1b_c35_r1_independent_verification.mjs`.
