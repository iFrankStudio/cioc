# C30-R2 — paid high-anchor calibration and measured selective dispatch
Parent ab9875fbe348467d57a084f20b49a1b4e77eea6a. Non-native engineering model only, no new native evidence. Original C30 evidence immutable. B3/B4 NOT_MET.

High-anchor arm sets ordinary cell40=16 via paid CIL and measures both parent and generated chain using that operand and current live y. Restores live x address via charged PatchConst before production. It avoids rejection from initial low x, but assumes declared bounded domain and validates only the calibration input. Selective arm has an additional mode dispatch: 0 parent,1 direct generated calibration,2 selective. Uses actual measured difference at x=16 and authored affine slope 2 (not learned from data) to derive cutoff 16-ceil(delta/2), by paid Sub/Branch. Stores it in executable Const with PatchConst. On each production, compare live x with cutoff via bounded countdown; restore x from input cell and zero accumulator before selecting parent or generated suffix. Cutoff inferred from pre-gate trial costs; **does not incorporate the selective dispatch overhead itself**, which is measured in lifetime comparison. Thus not an optimal threshold or independent learned cost model. Same-source off suppresses only final mode2 activation, all construction and measurement effects remain. Verify preactivation prefix, products, exact generated PC counts and address. Strict capacities retained. External input stable during construction/calibration/service, no horizon known to actor. No native claim.

16 revealed original cases x5 arms, 144 selected new engineering schedules (16 initial x,3 horizons, low/high/mixed later inputs) x5 arms =800 model worlds. All low/high/mixed schedules include actual paid production changes; no blind or population claim. Comparators plain original, R1 generic, high anchor, selective, selective activation off. Every emitted product matched plain. Actual measured cutoff4 in selective graph; 2x-8 measured pre-gate difference. All cost included, even unsuccessful local trials. 
| family | R1 net | high-anchor net | selective net | selective physical saving vs off |
|---|---:|---:|---:|---:|
|revealed prospective|4440|3224|-16842|-9562|
|grid|66666|72816|-198200|-106136|
|low|-26672|-31584|-87228|-56540|
|high|67268|75776|-35676|-4988|
|mixed|26070|28624|-75296|-44608|

Capacity and wins: ```json
{
  "known": {
    "generic": {
      "n": 12,
      "net": 4440,
      "wins": 8,
      "paid": 23682,
      "nodes": 109,
      "peak": 120,
      "clones": 6571
    },
    "high": {
      "n": 12,
      "net": 3224,
      "wins": 7,
      "paid": 24898,
      "nodes": 111,
      "peak": 122,
      "clones": 6694
    },
    "selective": {
      "n": 12,
      "net": -16842,
      "wins": 0,
      "paid": 44964,
      "nodes": 141,
      "peak": 152,
      "clones": 6007
    },
    "off": {
      "n": 12,
      "net": -7280,
      "wins": 0,
      "paid": 35402,
      "nodes": 141,
      "peak": 152,
      "clones": 0
    }
  },
  "grid": {
    "generic": {
      "n": 144,
      "net": 66666,
      "wins": 80,
      "paid": 364326,
      "nodes": 109,
      "peak": 120,
      "clones": 91548
    },
    "high": {
      "n": 144,
      "net": 72816,
      "wins": 80,
      "paid": 358176,
      "nodes": 111,
      "peak": 122,
      "clones": 104448
    },
    "selective": {
      "n": 144,
      "net": -198200,
      "wins": 0,
      "paid": 629192,
      "nodes": 141,
      "peak": 152,
      "clones": 95188
    },
    "off": {
      "n": 144,
      "net": -92064,
      "wins": 0,
      "paid": 523056,
      "nodes": 141,
      "peak": 152,
      "clones": 0
    }
  },
  "groups": {
    "low": {
      "generic": {
        "n": 48,
        "net": -26672,
        "wins": 0,
        "paid": 86552,
        "nodes": 109,
        "peak": 120,
        "clones": 6069
      },
      "high": {
        "n": 48,
        "net": -31584,
        "wins": 0,
        "paid": 91464,
        "nodes": 111,
        "peak": 122,
        "clones": 6888
      },
      "selective": {
        "n": 48,
        "net": -87228,
        "wins": 0,
        "paid": 147108,
        "nodes": 141,
        "peak": 152,
        "clones": 378
      },
      "off": {
        "n": 48,
        "net": -30688,
        "wins": 0,
        "paid": 90568,
        "nodes": 141,
        "peak": 152,
        "clones": 0
      }
    },
    "high": {
      "generic": {
        "n": 48,
        "net": 67268,
        "wins": 42,
        "paid": 153652,
        "nodes": 109,
        "peak": 120,
        "clones": 53039
      },
      "high": {
        "n": 48,
        "net": 75776,
        "wins": 48,
        "paid": 145144,
        "nodes": 111,
        "peak": 122,
        "clones": 60568
      },
      "selective": {
        "n": 48,
        "net": -35676,
        "wins": 0,
        "paid": 256596,
        "nodes": 141,
        "peak": 152,
        "clones": 60538
      },
      "off": {
        "n": 48,
        "net": -30688,
        "wins": 0,
        "paid": 251608,
        "nodes": 141,
        "peak": 152,
        "clones": 0
      }
    },
    "mixed": {
      "generic": {
        "n": 48,
        "net": 26070,
        "wins": 38,
        "paid": 124122,
        "nodes": 109,
        "peak": 120,
        "clones": 32440
      },
      "high": {
        "n": 48,
        "net": 28624,
        "wins": 32,
        "paid": 121568,
        "nodes": 111,
        "peak": 122,
        "clones": 36992
      },
      "selective": {
        "n": 48,
        "net": -75296,
        "wins": 0,
        "paid": 225488,
        "nodes": 141,
        "peak": 152,
        "clones": 34272
      },
      "off": {
        "n": 48,
        "net": -30688,
        "wins": 0,
        "paid": 180880,
        "nodes": 141,
        "peak": 152,
        "clones": 0
      }
    }
  }
}
```

The experiment separates calibration input from actual workload and explicitly charges dispatch. A measured crossover in the trial path is not sufficient for a profitable dispatch policy when the classifier changes the path cost. Do not interpret this as adaptive admission or proof of model accuracy beyond the prescribed graph. Next prioritize lowering dispatch cost (or changing generated representation) over adding per-query learning machinery; compare simpler high-anchor calibration and a bounded generic representation without per-service threshold loops. Freeze fresh native cases only after selecting a materially useful mechanism. B3/B4 NOT_MET.
Reproduce: node cl1b/analysis/cl1b_c30_measured_dispatch.mjs. Model executed in connector JS.
