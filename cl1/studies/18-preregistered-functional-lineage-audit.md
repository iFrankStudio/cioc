# CL-1 Study 18 — preregistered source-sensitive functional audit calibration

**Registered 2026-11-13, before any Study-18 implementation or outcome inspection.** Status: protocol only. This is an observer/auditor calibration, **not** a positive life experiment. Study 14's source-sensitive functional endpoint and direct external-restoration positive control remain NOT EVALUABLE; Studies 15–17 supply event-level cell and graph ancestry but do not retrospectively repair Study 14.

## Question and falsifiable target

Can a read-only auditor reconstruct a time-ordered, provenance-qualified dependency from a successful cell read of a descriptor, through graph construction or replacement and successful graph launch, to a subsequent output-cell write by the launched process? This is a **traceability** question, not a claim that the descriptor caused the output value, that the output is useful, or that any organization maintained itself.

## Frozen design

1. **No changes to the CIL instruction set, VM execution, scheduler, resource charges, random ecology or survival criterion.** Study 18 may add a read-only Python audit and minimal observer-only metadata only if existing successful-event records cannot disambiguate identity. Never synthesize missing provenance from temporal coincidence.
2. Audit the existing event schema using a deterministic synthetic **positive trace fixture** containing explicit successful descriptor read, graph encoding/replacement, launch and child output write, plus independently authored fixtures for (a) externally written descriptor, (b) unknown writer, (c) trapped encode/launch, (d) output written by another process, (e) incorrect event order and (f) identity reuse or missing graph ancestry. Synthetic fixtures are mechanism controls, **not biological replicates**.
3. Predefine a strict chain as ordered successful events with stable graph/process/cell identity and verified actor provenance at every hop. The descriptor writer must be an internal process event. The graph's effective executable code must be linked to that descriptor; graph launch must identify the actual child; the child must subsequently read an identified input and write an identified output. If an existing event lacks a needed link, label the chain **NOT EVALUABLE**; do not infer it from a shared owner or adjacent timestamps.
4. Report separate counts for: complete traceable chains; chains with external descriptor writers; unknown provenance; trapped operations; and structurally broken chains. Never relabel an external origin as endogenous merely because a later VM action is internal.
5. Reuse existing archived Study 14–17 datasets **only as retrospective instrumentation checks**, with their original frozen endpoints and denominators unchanged. Do not combine cohorts or search for favorable seeds. A future **new** ecological study must preregister fresh independent seeds, matched untreated/sham/single- and double-ablation arms, a budget-matched explicit external repair arm and a zero-credit control **before** execution.
6. For that future study, functional dependence must be established by paired intervention on two internally originated components and a prespecified input-output transformation measured in at least two separated post-lesion windows. A trace chain alone, graph recreation alone, or successful external restoration is insufficient. All nonexposed and failed worlds stay in the denominator.

## Technical acceptance and audit

- Independent Python auditor reads SQLite without modifying it; machine-readable report includes source revision, schema version, explicit missing-link reasons and per-trace event IDs.
- Positive fixture is accepted only when every link is explicit; all six negative fixtures must be rejected or classified not evaluable with a reason.
- Existing Rust tests, formatting, strict Clippy, Python unit tests, exact SQLite full-world and epoch replay and source-pinned GitHub CI must pass before calling the implementation verified.
- A future intervention must independently demonstrate that external restoration is capable of restoring the *registered function*; if the control cannot do so, the restoration endpoint is **NOT EVALUABLE**, never zero or positive.
- **Stopping rule:** record negative and not-evaluable outcomes without retuning criteria or seeds. No claim of organizational closure, autonomous repair, reproduction or cyberlife follows from Study 18.

## Scientific boundary

Study 18 resolves whether the instrumentation can support a prospective causal functional experiment. It does not test CL-1's positive milestone and cannot be used to declare CL-1 accepted.
