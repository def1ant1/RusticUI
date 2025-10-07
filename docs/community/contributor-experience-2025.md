# RusticUI Contributor Experience Survey 2025

> **Purpose:** capture actionable, automation-friendly insights that keep RusticUI enterprise-ready while minimizing manual toil for maintainers and adopters alike.

## Research Goals

1. **Quantify onboarding friction** across tooling, docs, and automation so we can prioritize workflows that merit additional generators or CI guardrails.
2. **Identify high-impact automation requests** that shrink repetitive chores (scaffolding, release hygiene, support playbooks) for internal platform teams.
3. **Benchmark satisfaction** with governance touchpoints—issue triage, PR reviews, and synchronous forums—to confirm our processes scale without bottlenecks.
4. **Surface ecosystem blockers** (framework support, performance, accessibility) that must be addressed before enterprise rollouts.

The 2025 study runs continuously from 2024-12-02 through 2025-03-28 so we can compare pre- and post-roadmap feedback. Quarterly snapshots will be exported for the governance retrospective alongside operational metrics (CI flake rate, release latency, NPS trend).

## Survey Instruments

| Theme | Question | Response Type | Automation Notes |
| --- | --- | --- | --- |
| Onboarding | “Which setup steps took longer than expected?” | Multi-select + free text | Connect to issue labeler to auto-file docs/tooling tasks based on top picks. |
| Automation Coverage | “Where would additional `cargo xtask` commands remove manual work?” | Ranked choice | Export to GitHub issue templates tagged `automation-wishlist`. |
| Documentation | “Rate the clarity of the quick-start blueprints (1–5).” | Likert scale | Mirror data into docs analytics dashboard for longitudinal trend. |
| Support | “How satisfied are you with PR review cadence?” | Likert scale | Trigger Discussions reminders when scores fall below 3. |
| Ecosystem Fit | “Which frameworks do you need first-class support for in 2025?” | Multi-select | Pipe to roadmap board swimlanes to adjust staffing. |
| Open Feedback | “What is the one thing we must fix before production?” | Long form | Feed into sentiment classifier to prioritize emergent risks. |

A GitHub Discussion form mirrors the core instrument for contributors who prefer staying inside GitHub: `https://github.com/apotheon-ai/rusticui/discussions/new?category=contributor-experience-2025`. The form enforces the same schema and syncs into the survey warehouse via webhook automation.

## Automation & Tooling Pipeline

1. **Primary collection** happens in the Typeform survey (`https://form.typeform.com/to/rusticui-cx-2025`). Enable authenticated sessions plus hidden fields (`github_handle`, `team_size`, `primary_framework`) so cross-tool matching is deterministic without manual spreadsheets.
2. **Dual ingestion** is handled by the `contributor-experience-intake` GitHub Action (nightly cron). The workflow:
   - Calls the Typeform Responses API with incremental cursors.
   - Fetches new GitHub Discussion form submissions via GraphQL.
   - Normalizes payloads into the shared JSON schema stored under `docs/data/contributor-experience-2025/`.
3. **Warehouse sync** writes the merged dataset to an append-only Google Sheet, plus rotates quarterly CSV snapshots into the `archives/research/2025Q*/` folders for reproducibility.
4. **Insight generation** relies on the `cargo xtask research-report` command (shipping in Q1). It generates:
   - Aggregated dashboards (NPS, automation requests) pushed to the RusticUI Project board widgets.
   - Issue/PR recommendations filed automatically when thresholds breach governance guardrails.
5. **Privacy & retention** – Purge raw PII after 180 days. Aggregated metrics remain indefinitely for longitudinal tracking.

## Operational Cadence

- **Weekly triage (Mondays 16:00 UTC)** – Governance working group reviews net-new responses, accepts auto-filed issues, and updates survey health metrics.
- **Monthly deep dive (First Thursday)** – Maintainers walk through trend reports, adjust roadmap swimlanes, and post executive summary to Discussions.
- **Quarterly retrospective** – Aligns with release planning to fold survey findings into the improvement roadmap and publish a public report.

Use the `projects/apotheon-ai/rusticui/6` board to visualize progress. Columns automatically populate via the nightly workflow, flagging:

- Survey instrumentation maintenance.
- Open feedback items awaiting action.
- Automations queued for implementation.

> **Note:** Automation-first delivery is mandatory—avoid manual exports or spreadsheet gymnastics. Extend the ingestion workflows if new questions are added.
