# CL-1B B20 — resource-request threshold and inaccessible feedback

**Evidence level:** independent source-shaped JS model and audited Rust scheduler semantics, **not an observed native Rust test result**. The [pre-registration](../studies/B20-resource-shortage-observability.md) was committed before the [B20 Rust integration test](../../cl0/crates/world/tests/cl1b_b20_resource_feedback_barrier.rs) and [repeatable observer model](../analysis/b20_resource_feedback_barrier.mjs) were implemented.

## Seven-world fixed-resource experiment

The exact B19 five-instruction resource-acquisition loop starts with **two local credits** and an empty world pool. The first Const and then a charged deferred Draw consume those two credits. All subsequent opportunity comes from **explicitly external, identity-neutral pool packets**, while actual CIL Draw/Write operations must pay one compute credit per attempted instruction.

| Requested Draw quantity | External packet schedule | Actual simulated PC4 service writes | Final pool | Final PC | Condition |
| ---: | --- | ---: | ---: | ---: | --- |
| 2 | [4,4,4,4] | **0** | **13** | 4 | One draw succeeded; actor then ran out of local credits *before service Write* |
| 3 | [4,4,4,4] | **4** | 0 | 1 | Four services; 3 transferred credits cover exactly three post-draw instructions |
| 4 | [4,4,4,4] | **3** | **1** | 1 | First packet4 below required draw threshold5; later packets accumulate |
| 2 | [5,5,5,5] | **0** | **17** | 4 | More available pool cannot run PC4 because only PC1 Draw can be pool-funded |
| 3 | [5,5,5,5] | **5** | 0 | 1 | Four packets fund five services via accumulated remainder |
| 4 | [5,5,5,5] | **4** | 0 | 1 | Extra transferred credit often gets spent on a subsequent deferred Draw |
| 4 | none | **0** | 0 | 1 | Closed finite-credit actor blocked without environmental supply |

All results are **source-shaped interpreter predictions** over finite opportunity windows, not native acceptance. The authored program/request amounts were chosen by investigators; the individual did **not** change its request or learn a policy.

## Two distinct deadlocks

**Threshold deadlock:** Request4 at zero local credits with pool4 is not eligible to execute Draw because the pool must pay *both* one instruction fee and the requested4-unit transfer. The world scheduler returns **no native step or internal event**. A later neutral packet pushes the pool above threshold and execution resumes; the individual did not detect the earlier shortfall.

**Post-draw debt deadlock:** Request2 succeeds and transfers 2 credits to its own process, but PC2 and PC3 consume both, leaving PC4 (the real service Write) without an instruction credit. The pool can subsequently grow to **13 or 17 credits**, yet the actor is ineligible to execute PC4 because the opt-in pool-funded rule applies *only to DrawCompute*. This is a sharper limitation than merely waiting for a threshold to be crossed.

## Narrow source-grounded impossibility

For an already blocked **single process** with zero local credits, the current CL-0 scheduler allows no ordinary CIL instruction to execute, even one that would modify the acquisition policy. An insufficient Draw is not an in-actor conditional observation or branch: when charged, `DrawDeferred` is emitted for **observer provenance**, and when the local credit balance is zero with inadequate pool, no instruction or event occurs at all. Without preplanned funded logic, an external code/credit intervention, or another funded process, the actor **cannot reactively rewrite its own request from the blocked state**.

The result is conditional on CL-0's current execution law; it does **not** prove that no alternative digital substrate could implement general adaptive resource acquisition. The appropriate research next step is to test *generic* CIL-visible resource failure feedback or self-funded multi-process acquisition management, not grant a favored organism arbitrary free credits.

**Scientific status:** B20 identifies a design obstacle and a feasibility tradeoff for native compute acquisition. It neither establishes endogenous fitness selection nor confirms that continuous cyberlife has formed. The Rust differential, once executed and archived with source SHA, must establish exact native scheduler semantics before promoting numerical results.
