# CL-0 Stage 4 — Independent Python analysis

Pure Python 3.10+ standard library. The reader opens SQLite with `immutable=1&mode=ro`, does not modify experimental records, and makes no organism/fitness claims.

```bash
cd cl0
python3 -m unittest discover -s analysis/tests -v
python3 analysis/analyze.py experiments.sqlite --json console/public/example.json --csv report.csv
```

The analysis compares matching initial SHA-256 snapshots. Numeric contrasts are **descriptive** only: they have no causality or statistical significance unless conditions and replicates were preregistered. SHA-256 checks detect inconsistent records but do not authenticate maliciously rewritten databases. An experimenter's intervention is not evidence of emergence.

The resulting JSON is accepted by the Vue observer console. No network access to the running world is needed.
