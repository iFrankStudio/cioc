# CL-1 event-driven Codex runner — operator checklist

This automation is **opt-in**: the `CL-1 event-driven Codex research` workflow will fail closed until repository secret `OPENAI_API_KEY` is configured. ChatGPT Plus does **not** pay for API usage; use a dedicated OpenAI API project/key with a spending ceiling.

## Activate

1. Create a dedicated OpenAI Platform API project and API key, with a small budget/alerts.
2. In GitHub **iFrankStudio/cioc → Settings → Secrets and variables → Actions → New repository secret**, create `OPENAI_API_KEY`. Never commit or paste the key into chat.
3. Verify repository Actions policy allows `openai/codex-action@v1` and that the workflow's `contents: write` and `actions: write` permissions are allowed for publish/dispatch.
4. Open **Actions → CL-1 event-driven Codex research → Run workflow** on `main` with `round=0`, once, to bootstrap. Completion of **CL-1 Study34 targeted retrospective audit** will also trigger the workflow.
5. Monitor the first run and cost. Keep the hourly ChatGPT research automation as fallback, not as proof that GitHub's runner is active.

## What happens per chain

- Trigger: manual `workflow_dispatch` or the completed Study34 CI workflow.
- `propose`: GitHub read-only token, unprivileged Codex action with its API credential held behind the official proxy; emits a patch artifact, not a push.
- `verify`: no API key or write token. Reapplies the patch, checks file scope, then runs Rust tests/lint, Python tests, full Study33 native SQLite replay and independent Study33/Study34 audits.
- `publish`: only after verifier success, checks original base SHA still matches `main`, applies the *same patch*, validates allowlisted paths a second time, commits and pushes with a separate write-scoped GITHUB_TOKEN.
- `dispatch`: after publishing, immediately starts the next pass using a GitHub `workflow_dispatch` event. The chain stops after **four runs (rounds 0..3)**. No extra PAT is needed for dispatch.
- A proposal with no changes, failed verification, an outdated main, or missing credentials stops the current chain. This is fail-closed, not an indefinite unattended repair service.
- Historical registration/results Markdown files are **append-only** for the autonomous agent; existing documents cannot be edited. The patch cannot edit workflows, secrets, dependencies, or kernel source. Up to eight research files and 180 KiB diff per proposal.

## Limits and operational caveats

- This does **not** wake an existing ChatGPT chat session. It launches separate Codex API executions in GitHub Actions.
- A CI pass means the programmed gates passed, **not** that native cyberlife has emerged. Existing Study33/Study34 null results stay visible.
- `workflow_run` has a native GitHub chaining limit; next-round chaining uses the explicitly supported `workflow_dispatch` exception for GITHUB_TOKEN events.
- An auto-pushed commit using GITHUB_TOKEN does **not** trigger ordinary push workflows. This chain performs its own mandatory verification before publishing.
- Consider switching to pull-request-only publishing for larger, high-impact changes. This initial version intentionally restricts automated scope and uses direct-main pushes only behind independent verification.
- Disable via **Actions → CL-1 event-driven Codex research → ... → Disable workflow**, or remove the API secret to stop Codex API calls. Spending should additionally be controlled at OpenAI Platform.

## Scientific release gate

No project milestone is considered reached unless independent preregistered cohort replication confirms internally sourced multicomponent causal dependence, endogenous functional recovery and unassisted cross-window maintenance. A successful Actions job never substitutes for this criterion.
