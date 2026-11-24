# C31-R2 — exact paid input-change admission
Parent 32d490b4d522eb60290e7759f378d589269bb3e5. Non-native model only. B3/B4 NOT_MET. R1 periodic structural search expensive. R2 adds four paid snapshot cells54..57. After initial generation, copy current inputs into snapshot then search both independent orientations using R1 actual ReadNext path comparison. Full-event policy after each output reads snapshot and live values in order, branches on first mismatch, copies all4 live values and re-searches both routes on mismatch; otherwise no search. Sampled policy uses R1 16-service countdown then same exact check (can miss transients; checks only actual current input, not intervening host updates). No host event notification, future information or free comparison. Both policies retain route patches while unchanged; both paid activation effects suppressed in matched off arms. Exact initial preactivation prefix compared, on/off decisions identical since unchanged graph and input schedules. All paid instructions, 4 snapshot cells, monitor reads, copies, searches and route patches charged. No numerical trial or R10 certificate; observer checks outputs, actual entry address and clone count and exact decision. Fixed grammar/cadence and monitor authored. Caps16 cells/200 nodes/220 occupied/30000 credit. Monitor cost includes unchanged countdown reset, searchPaid counts snapshot and local search through before finish. These are diagnostic subsets of total cost, not complete independent economic terms.

24 revealed R0/R1 scenarios ×8 runs =192 model worlds, no blind holdout. Baselines plain, R1 fixed DD, initial-only, periodic16; event on/off, sampled event on/off. All outputs exact, event monitor count n-1, actual snapshot count equals exact change count plus initial (checked independently); sampled monitor only every16. 
| family | DD net | initial net | periodic net | event net | event off | sampled net | sampled off |
|---|---:|---:|---:|---:|---:|---:|---:|
|all|57256|57688|41554|-44370|-49758|43642|39706|
|left|232|-80|-3026|-12636|-12636|-2610|-2610|
|right|19432|20872|17746|8316|6516|18342|16542|
|conflict|9232|10144|7288|-2412|-3612|7614|6414|
|flip|8782|7870|6556|-3130|-4330|6693|5757|
|alternating|9704|9392|6446|-4986|-5874|6862|6862|
|mixed|9874|9490|6544|-29522|-29822|6741|6741|
Event monitoring 1776 checks, 358 snapshots, 62568 monitor instructions, 43372 snapshot/search instructions. Sampled 90 checks, 30 snapshots, 3564 monitor instructions, 3240 snapshot/search instructions. Physical effect = matched off minus on total. No native promotion. If overhead dominates, stop external input novelty detection and reuse already observed production path information, or move to richer workload with larger selection opportunity; no claim of evolving new grammar. B3/B4 NOT_MET.
Reproduce node cl1b/analysis/cl1b_c31_exact_change_admission.mjs.
