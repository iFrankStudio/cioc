# CL-0 Observer (Vue 3 + TypeScript)

This is a **read-only**, client-side experimental observer. It never sends commands to the Rust world.

```bash
cd cl0/console
npm install
npm run build
npm run dev
```

Generate data:
```bash
cd cl0
python3 analysis/analyze.py experiment.sqlite --json report.json
```
Then open the observer and select `report.json` via the file input. No HTTP backend, storage permissions, authentication or online account needed. Experiment records remain on the local machine. This app **does not** declare any observed process alive or self-organizing.
