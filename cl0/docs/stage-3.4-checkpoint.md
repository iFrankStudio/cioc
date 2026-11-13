# CL-0 Stage-3.4 checkpoint — 2026-11-13

## Implemented
- Graph owner-controlled per-process read authorization, explicit revocation, opcode tag introspection, and read-only single-successor-edge introspection.
- Deterministic scheduled intervention runner with multiple matched conditions sharing an identical world snapshot and explicit schedule validation.
- Durable SQLite `scheduled_runs` records and exact same-version replay verification.
- Negative controls for unauthorized reads, timed disable-before-write, unreachable scheduled actions, invalid schedule, reopening persisted DB, and tampered final state.

## Observed failures and limitations
- The first read-rights test interleaved the owner's revocation with the reader's operations and would not prove a successful granted read. The fixture was changed to freeze the owner with an explicitly logged external credit intervention until the read completes, then resume the owner for revocation. This adjustment is a **test-design repair**, not evidence of autonomous permission negotiation.
- Reader grants are explicitly not storage-priced. Only opcode tags and one successor edge can be introspected; no full opcode operands are yet exposed.
- The scheduled runner records absolute offsets from the initial branch in attempted CIL instructions, not real time. Intermediate world states are reconstructible but are not individually persisted yet.
- Constructed fixtures demonstrate instrumentation capabilities, not life, spontaneous closure, self-maintenance, or open-ended evolution.

## Verification
Verified with GitHub Actions run https://github.com/iFrankStudio/cioc/actions/runs/43402811526 at commit `2095b5e`: **35/35 Rust tests passed** (11 lab + 24 world), strict Clippy passed. This run covers the scheduled intervention table, disk reopen/tamper controls and graph read permissions. The CI formatting step still runs `cargo fmt --all` rather than enforcing `--check`.
