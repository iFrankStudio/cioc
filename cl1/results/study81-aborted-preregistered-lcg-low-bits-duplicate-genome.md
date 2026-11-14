# CL-1 Study81 prospective ecology — invalid generator, stopped before its registered population

**Status: INVALID POPULATION DESIGN / ABORTED. NOT an ecological result.**
Registered source protocol: `cl1/studies/81-preregistered-stochastic-native-executable-edge-mutation-survey.md`. Native opcode-tracing targeted calibration passed both focused Rust tests, but the original prospective run did **not** pass. GitHub Actions run [43592644179](https://github.com/iFrankStudio/cioc/actions/runs/43592644179), source revision `51dcd65940ab5e81db61777ad9d68c7d89882799`, posted GitHub commit CI status **failure**. The laboratory runner stopped with exactly:

```
Error: "duplicate seeded code genome: 5400026"
```

The original SQLite artifact from this failed run was nevertheless archived as GitHub Actions artifact **14112544240**, ZIP SHA256 **6d0627f326c0392fd01f0f68bcc978fbb94f418362ab0960117393af194eed1f**. That **partial, failed** corpus must not enter any verified 128-genotype denominator or be treated as successful reproduction. No Study81 independent Python 512-run/12,288-hash audit ran because the Rust command stopped at the duplicate.

## Scientific failure mode

Study81 chose random opcode bins via a 64-bit LCG followed by `% 8`; modulo a power of two exposes the LCG's shortest-period low bits instead of well-mixed random output. The generator also consumes a variable extra LCG draw only on selected opcode bins, inducing further code-grammar correlation. A fixed-seed generator that produces duplicate full programs cannot support Study81's *registered 128 different initial genomes* assumption. This is a generator-design/experimental-identifiability problem, **not** evidence for or against cyberlife or native functional self-repair. It is not appropriate to exclude duplicates, replace seeds selectively, modify the Study81 sampler in place after the failed run, or count repeated code as independent genotype observations.

## Next study decision, before any next ecological outcome

Keep the failed Study81 record immutable and separately preregister a new Study82 population using nonoverlapping untouched seeds and a **per-draw SplitMix64-mixed output** rather than an LCG low-bits modulo bin. Include the same four paired world laws, frozen code length/resources/epochs, original code-genome uniqueness gate, native before/after edit and source-def-use provenance, complete 512 worlds and 12,288 source-pinned SHA256 epoch snapshots. A new sampler changes the sampled genome distribution; it must be reported as a new population, **not** a successful retry of the original Study81 outcome or a replication of naturally repaired code. No lesion/recovery experiment may be inferred.

**CL-1 remains OPEN.**
