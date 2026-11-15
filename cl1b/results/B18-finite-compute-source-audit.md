# CL-1B B18 — finite compute conservation and a necessary energy-flux gate

**Evidence level:** direct audit of the committed CL-0 Rust VM resource accounting and a mathematical upper bound for the stated world model. **Native B18 test has been committed but no successful source-pinned CI run has been verified.** This is an architecture constraint, not a cyberlife emergence result. [Protocol](../studies/B18-finite-compute-no-perpetual-maintenance.md) · [native regression test](../../cl0/crates/world/tests/cl1b_b18_finite_compute_bound.rs).

## Source-verified accounting facts

In `cl0/crates/world/src/lib.rs`:

- `World::step()` chooses an eligible native CIL process and charges exactly one compute credit per attempted opcode, including trap or halt. Under ordinary execution, it decrements the process's local `credits`; in opt-in `pool_paid_draw` mode, it may pay that charge from `compute_pool` instead.
- `World::execute_compute()` implements `DrawCompute`, `ReturnCompute` and `TransferCompute` as **checked transfers** among already-existing balances. None creates a credit; an attempt's ordinary instruction charge remains payable.
- `GraphLaunch` transfers charged compute from a parent into a child. New process creation does not make compute inventory appear from nowhere.
- Researcher-side `Intervention::SetCredits` and `Intervention::SetComputePool` **can** change total compute inventory, but those events have explicit `Origin::External` provenance; they are not native CIL life behavior. The initial `Process::new(..., initial_credits)` loaded into the world is also an external endowment.

Let total usable compute inventory at step \(k\) be

\[
E_k=P_k+\sum_{i\in\mathrm{processes}}c_{i,k}.
\]

For each successful native scheduler attempt in a closed world with no new host interventions and with no additional externally supplied resources, **\(E_{k+1}=E_k-1\)**. Since all balances are nonnegative integers,

\[
N_{\text{native attempts}}\le E_0.
\]

This is a **necessary conservation bound for the current CL-0 substrate**, independent of the sophistication of a CIL algorithm: coding a self-repair loop, a recursive graph copier or a resource-transfer circuit does not make perpetual computation possible. Traps, halts, inaccessible pool balance, or unusable process states may reduce attempts below \(E_0\).

## Frozen B18 native controls and source-derived predictions

| World | Initially supplied compute | Source-level predicted continuation |
| --- | ---: | --- |
| FINITE_SINGLE | one process10, pool0 | exactly10 charged `Const` self-loop steps, then zero eligible work |
| FINITE_POOL_CYCLE | process4, pool5, opt-in pool-paid Draw | 6 charged attempts; one pool-paid Draw; charged unsuccessful Return ends the actor, **3 stranded pool credits** |
| FINITE_TWO_ACTORS | process1=6, process2=4, pool0 | at most10 total charged attempts across both; native `TransferCompute` reallocates credits but cannot mint them |
| EXTERNAL_REFILL_POSITIVE_CONTROL | process5, pool0 | exactly5 work attempts, then zero; **externally attributed** `SetCredits(1,5)` enables five additional attempts, not internally originated |

These are **precommitted expected native test results**, not observed Rust CI passes. B18 [source](../../cl0/crates/world/tests/cl1b_b18_finite_compute_bound.rs) asserts the total-inventory charge invariant **after every native attempt** and checks intervention provenance.

## Interpretation and necessary future substrate condition

B00–B17 fixed a finite local credit budget for 80–120 opportunities and often provided no usable environmental compute flux. Even a fully correct continuous self-repair mechanism cannot prove indefinite maintenance within that experimental physics. It can prove at most **finite-horizon self-repair and finite-horizon functional continuity**. Claiming a truly persistent entity solely because it remains runnable at the endpoint would mistake **remaining fuel** for self-sustaining metabolism.

**Do not reward a specially designated individual with host-assigned compute and call that metabolism.** The scientifically useful next environment is an *identity-neutral, protocol-visible resource flux* (e.g., new environmental compute available to any actor capable of paying to access it), with budgeted native `DrawCompute`, access competition and observer accounting. Resource availability can be externally defined without hardcoding any native organism or reward function. The individual must demonstrate it can **earn or acquire access through its own code and survive held-out supply variation**. A replenished process that merely consumes endless free credit still fails the adaptive autonomy gate.

If the world is later amended to admit exogenous inflow \(I_k\), the correct conservation bound becomes \(N\le E_0+\sum_k I_k\). This admits unbounded operation only if supply is itself unbounded over the observation horizon **and** the actor remains able to access it; influx alone does not ensure life.

**Conclusion:** B18 exposes a foundational *feasibility precondition*, not an affirmative finding that CL-1B has created cyberlife. The current scientific acceptance standard must require explicit environmental inflow and evidence of actor-side acquisition before making any claim of ongoing self-maintenance.
