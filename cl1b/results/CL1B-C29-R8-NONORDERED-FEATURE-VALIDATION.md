# C29-R8 — nonordered feature verification and feature-alias counterexamples

Parent c6e3a7f2dbd2a95acfb258e2855a36ad8ad9c463. **Non-native component model only**, no new native or physical edit evidence. Original native C29 unchanged; B3/B4 NOT_MET.

## Mechanisms

Kernel has ordinary indirect Read/Write, but no bitwise operation in Op. Do not assume a free host bitmap. R7 shared capture retained. Compare five strategies: paired, periodic4, ordered cursor, recent feature, indexed feature. All use the same training/validation/fallback when applicable. Actor checks captured x (cell40). Indexed: compute address70+x by charged Const/Read/Add, read marker, audit only if absent/zero. Successful actual paired residual check writes 1 to that address. No hidden table or free lookup; capacity counts every allocated cell (up to 16 additional cells). Recent: reuse phase47 as last successfully audited x+4 (initial sentinel21); if current x differs, audit; on success store x+4. No additional cells. Both perform actual measured output on audit and sticky trust revocation on nonzero residual; no change to candidate bodies during one world. Both continue to pay for affine inference before checking feature. Neither revalidates a constant feature after a successful audit unless other observed x values cause recent eviction. Both use only x, not y; evidence about x is not proof over all y. No full trust certification or adaptive feature discovery. Cost formulas outside actor verified against actual paired reference on every emitted query; all trial products checked. Data ordinary world memory, no private access protection claim. Native Op compatibility inspected; native capacity NOT tested.

## Experiments

7 profiles: cross, unroll, switch5/11/16 (x-specific branch), new yswitch5/11 (same correct multiplication, switch B from cross-loop to x-loop at y-specific branch). The switch decision charges 5 instructions in B. All anchors and validation unchanged. Profiles are authored, not generated. 12 deterministic R6 grid schedules per profile, plus 5 stress schedules for each of 5 branch-specific profiles (missing-first, clustered, rare-first, missing-middle, feature-alias). 109 profile-schedule combinations ×5 policies =545 model worlds, 34880 outputs. Retrospective original schedules plus new deliberately constructed feature-alias stress cases, not a blind test or representative distribution. Environment coherent within query. External equal-output cutoff, no future horizon guest input. All measured compute charged, setup per batch included. No productive service or edits in this component.

| group/profile | policy | errors | sign errors | revocations | audits | net vs paired | nodes/peak | marker cells |
|---|---|---:|---:|---:|---:|---:|---:|---:|
|grid/cross|paired_0|0|0|0/12|0|0|177/190|0|
|grid/cross|validated_4|0|0|0/12|192|-3675|205/222|0|
|grid/cross|ordered|0|0|0/12|319|-18913|209/226|0|
|grid/cross|recent|0|0|0/12|768|-73836|207/224|0|
|grid/cross|indexed|0|0|0/12|192|-5019|209/242|16|
|grid/unroll|paired_0|0|0|0/12|0|0|183/196|0|
|grid/unroll|validated_4|0|0|0/12|0|-7848|211/228|0|
|grid/unroll|ordered|0|0|0/12|0|-7848|215/232|0|
|grid/unroll|recent|0|0|0/12|0|-7848|213/230|0|
|grid/unroll|indexed|0|0|0/12|0|-7848|215/232|0|
|grid/switch5|paired_0|0|0|0/12|0|0|186/199|0|
|grid/switch5|validated_4|24|6|5/12|127|-3987|214/231|0|
|grid/switch5|ordered|22|5|6/12|84|1087|218/235|0|
|grid/switch5|recent|0|0|12/12|130|-18875|216/233|0|
|grid/switch5|indexed|6|2|10/12|120|-14657|218/251|16|
|grid/switch11|paired_0|0|0|0/12|0|0|186/199|0|
|grid/switch11|validated_4|39|24|1/12|178|-1221|214/231|0|
|grid/switch11|ordered|25|15|6/12|104|1726|218/235|0|
|grid/switch11|recent|0|0|12/12|122|-18415|216/233|0|
|grid/switch11|indexed|6|4|10/12|113|-13942|218/251|16|
|grid/switch16|paired_0|0|0|0/12|0|0|186/199|0|
|grid/switch16|validated_4|24|23|5/12|131|-2394|214/231|0|
|grid/switch16|ordered|27|26|4/12|127|438|218/235|0|
|grid/switch16|recent|0|0|12/12|110|-17443|216/233|0|
|grid/switch16|indexed|3|3|11/12|100|-14831|218/251|16|
|grid/yswitch5|paired_0|0|0|0/12|0|0|186/199|0|
|grid/yswitch5|validated_4|32|21|3/12|150|-2445|214/231|0|
|grid/yswitch5|ordered|24|16|5/12|102|631|218/235|0|
|grid/yswitch5|recent|0|0|12/12|138|-19623|216/233|0|
|grid/yswitch5|indexed|6|4|10/12|136|-16005|218/251|16|
|grid/yswitch11|paired_0|0|0|0/12|0|0|186/199|0|
|grid/yswitch11|validated_4|31|9|3/12|153|-2634|214/231|0|
|grid/yswitch11|ordered|14|6|10/12|70|-498|218/235|0|
|grid/yswitch11|recent|0|0|12/12|130|-18743|216/233|0|
|grid/yswitch11|indexed|6|2|10/12|113|-13786|218/251|16|
|stress/switch5|paired_0|0|0|0/5|0|0|186/199|0|
|stress/switch5|validated_4|41|0|4/5|28|-1546|214/231|0|
|stress/switch5|ordered|183|0|1/5|7|8184|218/235|0|
|stress/switch5|recent|0|0|5/5|8|-2533|216/233|0|
|stress/switch5|indexed|0|0|5/5|8|-2611|218/237|2|
|stress/switch11|paired_0|0|0|0/5|0|0|186/199|0|
|stress/switch11|validated_4|41|41|4/5|28|-880|214/231|0|
|stress/switch11|ordered|182|182|1/5|13|11016|218/235|0|
|stress/switch11|recent|0|0|5/5|8|-2623|216/233|0|
|stress/switch11|indexed|0|0|5/5|8|-2701|218/237|2|
|stress/switch16|paired_0|0|0|0/5|0|0|186/199|0|
|stress/switch16|validated_4|41|41|4/5|28|-325|214/231|0|
|stress/switch16|ordered|180|180|1/5|18|13224|218/235|0|
|stress/switch16|recent|0|0|5/5|8|-2698|216/233|0|
|stress/switch16|indexed|0|0|5/5|8|-2776|218/237|2|
|stress/yswitch5|paired_0|0|0|0/5|0|0|186/199|0|
|stress/yswitch5|validated_4|41|3|4/5|28|-1720|214/231|0|
|stress/yswitch5|ordered|185|32|0/5|0|9624|218/235|0|
|stress/yswitch5|recent|63|32|3/5|5|1741|216/233|0|
|stress/yswitch5|indexed|63|32|3/5|5|1479|218/236|1|
|stress/yswitch11|paired_0|0|0|0/5|0|0|186/199|0|
|stress/yswitch11|validated_4|41|0|4/5|28|-2530|214/231|0|
|stress/yswitch11|ordered|184|0|0/5|0|6315|218/235|0|
|stress/yswitch11|recent|63|0|3/5|5|553|216/233|0|
|stress/yswitch11|indexed|63|0|3/5|5|291|218/236|1|

Grid branch profiles: 60 worlds per policy. Indexed errors27, revocations51; recent errors0, revocations60; ordered112/31, periodic4 150/17. **This does not generalize to stress sequences**. Across the 25 branch stress worlds per policy, both indexed and recent emit126 errors (all on y-specific branch profiles). On the feature_alias test for y-specific branches, x stays8; y changes from4 to5 or11 after32 queries. The first audit succeeds and subsequent 32 queries emit wrong differences without revocation. No missing-x starvation is needed to create this failure. For x-specific profiles, indexed may audit a branch coordinate at x=y where the two candidate loop counts are equal and residual is zero, then skip later differing y. A feature marker denotes one successful sample, not proof for an equivalence class. Exact input-pair validation would be more costly and still only valid for unchanged candidate code/environment semantics.

Feature alias examples (observer diagnostic):
```json
[
  {
    "profile": "yswitch5",
    "policy": "recent",
    "paid": 5359,
    "audits": [
      {
        "i": 0,
        "x": 8,
        "y": 4,
        "pred": 7,
        "actual": 7
      }
    ],
    "errors": 32,
    "firstError": {
      "x": 8,
      "y": 5,
      "emitted": 4,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 0,
      "paid": 76,
      "exact": false,
      "signCorrect": false
    }
  },
  {
    "profile": "yswitch5",
    "policy": "indexed",
    "paid": 5487,
    "audits": [
      {
        "i": 0,
        "x": 8,
        "y": 4,
        "pred": 7,
        "actual": 7
      }
    ],
    "errors": 32,
    "firstError": {
      "x": 8,
      "y": 5,
      "emitted": 4,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 1,
      "paid": 78,
      "exact": false,
      "signCorrect": false
    }
  },
  {
    "profile": "yswitch11",
    "policy": "recent",
    "paid": 5935,
    "audits": [
      {
        "i": 0,
        "x": 8,
        "y": 4,
        "pred": 7,
        "actual": 7
      }
    ],
    "errors": 32,
    "firstError": {
      "x": 8,
      "y": 11,
      "emitted": -14,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 0,
      "paid": 94,
      "exact": false,
      "signCorrect": true
    }
  },
  {
    "profile": "yswitch11",
    "policy": "indexed",
    "paid": 6063,
    "audits": [
      {
        "i": 0,
        "x": 8,
        "y": 4,
        "pred": 7,
        "actual": 7
      }
    ],
    "errors": 32,
    "firstError": {
      "x": 8,
      "y": 11,
      "emitted": -14,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 1,
      "paid": 96,
      "exact": false,
      "signCorrect": true
    }
  }
]
```

Grid indexed missed-x examples:
```json
[
  {
    "profile": "switch5",
    "id": "ascending_0",
    "firstError": {
      "x": 5,
      "y": 9,
      "emitted": -17,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 16,
      "paid": 81,
      "exact": false,
      "signCorrect": true
    },
    "matchingAudits": [
      {
        "i": 4,
        "x": 5,
        "y": 5,
        "pred": -5,
        "actual": -5
      }
    ]
  },
  {
    "profile": "switch5",
    "id": "permuted_0",
    "firstError": {
      "x": 5,
      "y": 9,
      "emitted": -17,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 16,
      "paid": 81,
      "exact": false,
      "signCorrect": true
    },
    "matchingAudits": [
      {
        "i": 4,
        "x": 5,
        "y": 5,
        "pred": -5,
        "actual": -5
      }
    ]
  },
  {
    "profile": "switch11",
    "id": "descending_1",
    "firstError": {
      "x": 11,
      "y": 15,
      "emitted": -17,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 16,
      "paid": 117,
      "exact": false,
      "signCorrect": true
    },
    "matchingAudits": [
      {
        "i": 5,
        "x": 11,
        "y": 11,
        "pred": -5,
        "actual": -5
      }
    ]
  },
  {
    "profile": "switch11",
    "id": "permuted_0",
    "firstError": {
      "x": 11,
      "y": 15,
      "emitted": -17,
      "actual": -5,
      "trust": 1,
      "measured": false,
      "verifiedCells": 16,
      "paid": 117,
      "exact": false,
      "signCorrect": true
    },
    "matchingAudits": [
      {
        "i": 2,
        "x": 11,
        "y": 11,
        "pred": -5,
        "actual": -5
      }
    ]
  }
]
```

Capacity/economics: on correct cross profile indexed costs5019 more than paired (peak242, 209 nodes, 32 allocated cells); recent costs73836 more, peak224, 207 nodes, 16 cells. All nonordered audited graphs exceed prior 200-node/220-total constraints, indexed can reach251 total. Recent audits every query on cross grid because x changes each query. A reduction in errors is not economic success; wrong estimates cannot be counted as net savings. No native promotion or capacity relaxation. Aggregate metrics for localized failure profiles:
```json
{
  "grid": {
    "paired_0": {
      "errors": 0,
      "revoked": 0,
      "net": 0,
      "worlds": 60
    },
    "validated_4": {
      "errors": 150,
      "revoked": 17,
      "net": -12681,
      "worlds": 60
    },
    "ordered": {
      "errors": 112,
      "revoked": 31,
      "net": 3384,
      "worlds": 60
    },
    "recent": {
      "errors": 0,
      "revoked": 60,
      "net": -93099,
      "worlds": 60
    },
    "indexed": {
      "errors": 27,
      "revoked": 51,
      "net": -73221,
      "worlds": 60
    }
  },
  "stress": {
    "paired_0": {
      "errors": 0,
      "revoked": 0,
      "net": 0,
      "worlds": 25
    },
    "validated_4": {
      "errors": 205,
      "revoked": 20,
      "net": -7001,
      "worlds": 25
    },
    "ordered": {
      "errors": 914,
      "revoked": 3,
      "net": 48363,
      "worlds": 25
    },
    "recent": {
      "errors": 126,
      "revoked": 21,
      "net": -5560,
      "worlds": 25
    },
    "indexed": {
      "errors": 126,
      "revoked": 21,
      "net": -6318,
      "worlds": 25
    }
  }
}
```

## Research decision

Stop expanding C29 fixed-feature validation heuristics. R4-R8 have established: measured parameters can generalize under an authored correct hypothesis; finite validation cannot certify arbitrary control-flow costs; fixed cadence aliases, ordered coverage starves, nonordered feature coverage aliases and imposes storage/compute debt. The next gate should change representation: explore a compact task-independent **cost-expression derived from actual executable structure**, with paid graph observations if possible, and explicit validity conditions; compare against always-paired and simple analytical baseline. Or investigate actor-generated equivalent arithmetic candidates using paid code edits plus independent functional correctness verification. Do not treat a feature class, a fit, or a finite set of observations as proof; do not continue tuning sampling on the revealed C29 family. Before claiming endogenous invention, require an actual new candidate body not preinstalled, actor-paid construction, measured correctness and benefit, matched controls. B3/B4 NOT_MET.

Reproduce: `node cl1b/analysis/cl1b_c29_nonordered_feature_validation.mjs`. Model body executed in connector JS; aggregate assertions and complete world outputs retained by script. No Rust CI.
