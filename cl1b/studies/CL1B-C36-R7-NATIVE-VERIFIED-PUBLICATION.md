# C36-R7 — verified publication and cached sensing

Parent f9515d5c293d62c93298c22f3d260dd4fc91c5ae; frozen test d18a500b9ef477904e141a2d1d234cb79584b432. STATUS: JS engineering preflight executed twice; Rust test NOT VERIFIED. R6 push SHA 3135d63174869901c78916af4164833ad3201b48: combined statuses and PR-only workflow query both empty. Local container cargo/rustc absent, github.com DNS fails. No CI conclusion inferred.

Existing native Op only; no kernel edits. Two actors with initial 220/15000 private credits, strict storage100, max nodes64, eight cells, neutral pool zero. Candidate mailbox12 separated from publication13; buyer checks candidate, pays 100 only if valid, returns ack; producer branches on ack, writes formal publication only after positive ack. Failed cached estimate triggers paid probe and retry for same offer. Fresh arm probes every offer; no_retry changes rejection branch to finish without publication; cache_zero changes buyer reward literal to 0. Different source arms, not matched physical edit suppression. Offset changes only between completed offers, not during validation/commit; shared cells are not isolated or authenticated. No reward on rejected candidate; no host credit refill. Combined inventory decreases one per attempted instruction. Test assertions require exact preflight counts and positive ack/correct value on every publication. Host reads are observer-only; only changes offer inputs at completion.

12 paired model worlds (3 schedules x4 arms), 24 offers each. Two independent JS implementations of same CIL subset gave identical counts (first exploratory run expected cache to complete fast and failed that expectation; retain observed negative result). Not independent Rust replay.

| regime | arm | completed | accepted | rejected | invalid candidates | probes | paid steps | producer end | buyer end |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
|stable|fresh|24|24|0|0|24|3311|820|11089|
|stable|cache|24|24|0|1|2|2325|1445|11450|
|stable|no_retry|24|12|12|12|1|2039|394|12787|
|stable|cache_zero|4|4|0|0|1|427|0|14793|
|mixed|fresh|24|24|0|0|24|3311|820|11089|
|mixed|cache|24|24|0|5|6|2797|1185|11238|
|mixed|no_retry|14|4|10|11|1|1227|0|13993|
|mixed|cache_zero|4|4|0|0|1|427|0|14793|
|fast|fresh|24|24|0|0|24|3311|820|11089|
|fast|cache|19|19|0|19|20|3999|0|11221|
|fast|no_retry|7|1|6|6|1|627|0|14593|
|fast|cache_zero|2|2|0|2|2|415|0|14805|

No incorrect formal publication in preflight; wrong candidates still occur. Cached sensing saves 986/514 total paid steps in stable/mixed, but exhausts in fast (fresh completes24). Negative evidence for universal cache superiority; do not increase starting credit to hide failure. Distinct candidate and publication addresses plus fixed offer stability, trusted verifier, protocol compliance, and no faults are assumptions. Sale may occur before publication; no transactional guarantee if actor exhausts between payment and commit. Buyer budget finite; ecosystem inventory declines. No self-modification or task-family discovery demonstrated. Next: verify R6/R7 native outcomes, archive traces and compare independent replay; investigate paid switch between fresh and cached paths only after mechanism gate. B3/B4 NOT_MET.

Reproduce native: `cd cl0 && cargo test --locked -p cl0-world --test cl1b_c36_r7_native_verified_publication -- --nocapture`.
