# C35-R3 — persistent quarantine

Parent 0456f60701803632d2bcab8ec0b0ea284c336114. On second alarm after a paid edge repair, guest patches dispatch to trusted parent, second write to publication, batch to direct pair. Both products recomputed before publication. Damaged nodes retained but not executed. Route-off suppresses dispatch patch only; matched prefix. Repair-off suppresses edge repair only.

205 non-native worlds, 2024 requests per arm. 24 repairable schedules and 17 opcode schedules (Add changed to Sub across all 16 clone positions).

| arm | repairable paid | wrong | repairs | quarantines | opcode paid | wrong | quarantines |
|---|---:|---:|---:|---:|---:|---:|---:|
|direct|120138|0|0|0|39273|0|0|
|unit1|135700|0|68|0|125385|412|0|
|quarantine|135700|0|68|0|49376|0|17|
|route_off|135700|0|68|0|35384|412|17|
|repair_off|128217|0|0|22|49376|0|17|

Quarantine correct on all scoped requests; no unnecessary isolation with active repair on repairable schedules. Route-off restores 412 errors. Repair-off also correct by abandoning optimized graph in 22 worlds. Quarantine not cheaper than direct. Trusted parent and checker remain assumptions. No native VM, CI, deletion, regeneration or B3/B4 promotion. Next stronger static baseline and path-local checks.
