# CL-1B B19 — identity-neutral environmental compute flux and native acquisition feasibility

**Evidence category:** independently implemented source-shaped JavaScript resource model, not native CL-0 Rust execution, and a source-grounded upper-bound argument. [B19 prospective protocol](../studies/B19-neutral-resource-flux-access.md) was committed before [native Rust tests](../../cl0/crates/world/tests/cl1b_b19_neutral_compute_flux.rs) and [observer simulation](../analysis/b19_compute_flux_model.mjs) results were inspected. **No actual passing native CI run or artifact has been verified.**

## Five-world result, event-indexed resource packets

One actor, five-node CIL founder, **two initially assigned local credits**, no compute pool at start. The researcher-created **world pool** receives resource packets externally (not individual-targeted credits). Every CIL attempt, including a deferred shortage or pool-funded Draw, costs one unit of total compute inventory. The genuine service event is an actor-internal `Write(cell9=1)` at PC4. There is no biologically meaningful wall-clock concept: the six windows are observation/supply opportunities.

| Arm | External pool packets | Native-like attempts in total | Actor-owned PC4 service writes | Unused pool at end |
| --- | --- | ---: | ---: | ---: |
| RENEWABLE_ACCESS | 5,5,5,5,5,5 | **32** (2 initial +30 packet-funded) | **6** | 0 |
| NO_DRAW_OPCODE | 5,5,5,5,5,5 | **2** | **0** | 30 |
| DRAW_NOT_POOL_PAID | 5,5,5,5,5,5 | **2** | **0** | 30 |
| SUBTHRESHOLD_THEN_CROSSING | 4,1 | **7** (2 initial +5 after second packet) | **1** | 0 |
| NO_ENVIRONMENTAL_INFLUX | none | **2** | **0** | 0 |

In the full-access arm the actor is **blocked, not stopped** with local credits0 at `DrawCompute` after each five-credit packet has been consumed. Each new neutral packet enables **one** pool-paid Draw and a fresh service write; a subsequent out-of-pool Draw is charged and deferred, again leaving the actor blocked. Thus the actor can remain *conditionally resumable* across discrete energy-availability episodes instead of depending on an investigator's targeted local-credit top-up.

The same number of external packets cannot sustain a same-size, researcher-authored program with no Draw opcode. Likewise, disabling the **generic** pool-paid Draw world law prevents access once the actor has spent all local credits. The size4 packet cannot fund the requested4 plus the necessary pool-paid instruction charge1; only the second packet crosses that threshold. This is a causal substrate-access distinction, not an actor's discovery of the optimum draw size.

### Causal accounting

Every resource packet is an explicit laboratory `Origin::External` `SetComputePool(current_pool + packet)`. Every Draw, deferred Draw and service Write is `Origin::Internal(1)`, with a resource charge. The independent simulator checked

\[
E_{\text{after native step}} = E_{\text{before}} - 1
\]

and

\[
E_{\text{after host packet}} = E_{\text{before}} + I.
\]

No resource creation from code, output, repair or pool transfer was inferred. Total available energy including the actor's initial endowment is **32** for the six-packet arm, and exactly32 native attempts were consumed. The no-access controls leave30 credits stranded in the environment.

The [native B19 integration test](../../cl0/crates/world/tests/cl1b_b19_neutral_compute_flux.rs) independently asserts these two conservation relations at each step/intervention, exact PC4 writes, internal-vs-external events, pool-paid draw provenance, deferred draws and no process births. **The Rust test has not yet been observed passing.**

## Scientific conclusion and limitations

**Source-level feasibility result:** the existing CL-0 world primitives can already *represent* identity-neutral compute influx and a continuing CIL actor's internally paid acquisition of that resource. It does **not** require a new `LIFE` or `METABOLISM` opcode, and native code can obtain compute without direct experimenter credits to its own process.

But a six-packet supply schedule is **finite**, and the acquisition instruction is **researcher-authored**. The host still controls when/how much new world compute becomes available. This demonstrates neither perpetual runtime nor unscripted self-organization. The B18 finite-inventory theorem continues to apply between external supply events.

**Next discriminating gate:** allow one continuing native organization to change its resource-acquisition policy in response to *held-out neutral resource availability patterns*, pay for its internal trials and policy edits, and compare with a same-source/same-endowment no-mutation control; ensure the service/evidence evaluator remains independent under code faults. Ideally include multiple processes competing for the **same** resource pool without host-side winner selection. Any positive outcome must be evaluated under new resource schedules not used to design the initial policy. Until then, B19 is a necessary physics feasibility calibration, not cyberlife.
