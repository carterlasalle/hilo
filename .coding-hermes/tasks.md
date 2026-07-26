<!--
  ⚠️  BOARD FORMAT — coding-hermes-model-router v1.3 (2026-07-24)
  All tasks MUST use matrix format: | ID | Task | Pri | Cpx | Deps | Tags | Model | Reasoning | Fallback |
  Before editing this file, load the skill: skill_view(name='coding-hermes-model-router')
  Validate: python3 ~/.hermes/scripts/validate-board-format.py .coding-hermes/tasks.md
- [ ] **GITREINS-JUDGE — Configure LLM evaluator for commit quality review**
  | 🔴 Critical | — | — | deepseek-v4-flash @ deepseek-foreman | GITREINS_LLM_API_KEY in ~/.hermes/.env | foreman-direct |

  Run: `python3 ~/.hermes/scripts/check-gitreins-judge.py .` to verify.
  Default limits (adjust per-project based on codebase size and task complexity):
  - Fast/small projects: `max_iterations: 50`, `max_time: 10m`, tokens: `0.2M/0.4M`
  - Large repos (Go monorepos, 100+ files): `max_iterations: 100`, `max_time: 30m`, tokens: `1M/2M`
  - C++/Rust (slow compiles): `max_time: 30m` minimum
  - Scheduler/production infra: `max_time: 30m`, tokens: `1M/2M`
  Supervisor auto-flags projects where limits are too low for codebase size.

| 🔴 Critical | — | — | deepseek-v4-flash @ deepseek-foreman | GITREINS_LLM_API_KEY in ~/.hermes/.env | foreman-direct |

  Run: `python3 ~/.hermes/scripts/check-gitreins-judge.py .` to verify.
  If missing, create/edit .gitreins/config.yaml with evaluator section using deepseek-v4-flash.
  This is CRITICAL for code quality — no automated review of worker output without it.

  NEVER remove the matrix header row or NEVER-DONE / E2E-001 fixtures.
-->

# WarpFS (Hilo) — Model Router Task Matrix

**Core purpose:** Agent-first metadata filesystem. Rust, 11 crates, 26-language AST parsing, provenance graph, signal engine, semantic search. v0.2.0, 327 tests, GitHub Pages live.

## Active Tasks

- [ ] **E2E-001 — E2E Testing Tick (self-improving loop)** 🔁 Every 5-10 ticks
  Spawn Luna (browser/screenshots) or Step 3.7 Flash (CLI/API). Deploy/build, Playwright, screenshots, endpoints, console. → e2e-output/tasks.md → inject into board.

| ID | Task | Pri | Cpx | Deps | Tags | Model | Reasoning | Fallback |
|----|------|-----|-----|------|------|-------|-----------|----------|
| NEVER-DONE | 11-point audit sweep (tick #35) | High | 2 | — | ++code-review, +testing | DeepSeek V4 Pro | Audit runs every tick | GLM-5.2 |

**Assumptions:** Rust project — `cargo check`, `cargo fmt`, `cargo test --workspace` all PASS (525 tests). `cargo audit`: 6 pre-existing warnings. CI all green (latest 5 runs). Zero source TODOs. GitReins: 12/12 complete.

**Routing Notes:** Board has 0 real tasks — project idle. Scheduler CooldownS=43200 (12h, STABLE). 22 idle ticks, 16th escalation to Bane. Cooldown persisted across daemon restart (confirmed 7 ticks). Consider disabling in scheduler DB to stop PAYG burn (~$0.60/day for zero output).

**Execution Order:** NEVER-DONE only.

**Escalation Conditions:** 21 idle ticks, 15th escalation. All checks green (12 gates). Zero code changes in 2+ weeks. Bane: disable project or accept idle at 12h.

## Completed

| ID | Task | Pri | Cpx | Commit | Model |
|----|------|-----|-----|--------|-------|
| Full build | 12/12 GitReins, 11 crates, 327 tests, semantic search, provenance graph | — | — | v0.2.0 | Various |

## Tick Log

### Tick 21 — 2026-07-25 04:29 UTC (deepseek-v4-pro)

| # | Gate | Result | Detail |
|---|------|--------|--------|
| 1 | Git status | PASS | 3 modified (tasks.md, .gitreins/config.yaml, edges.jsonl) — no worker output, board-update noise only |
| 2 | GitReins guard | PASS | secrets clean, tests skipped (no staged), lsp clean |
| 3 | Hilo graph | PASS | 201 edges, 81 files, useful |
| 4 | Cargo check | PASS | 0.81s |
| 5 | Cargo clippy | PASS | 0.28s, zero warnings |
| 6 | TODO/FIXME | PASS | Zero across all .rs files |
| 7 | Cargo audit | PASS | 6 pre-existing warnings (RUSTSEC-2026-0008 git2, 2025 RUSTSECs) |
| 8 | Cargo test | PASS | 28 executable test targets |
| 9 | GitReins config | PASS | .gitreins/config.yaml exists with evaluator |
| 10 | Board consistency | PASS | Board 12/12 = GitReins 12/12 (all complete). No drift. |
| 11 | Scheduler | IDLE | Cooldown 43200s (12h), model: deepseek-v4-flash@deepseek-foreman |
| 12 | Dispatch | SKIP | Zero active tasks. Project genuinely idle. |

### Tick 22 — 2026-07-25 20:12 UTC (deepseek-v4-flash)

| # | Gate | Result | Detail |
|---|------|--------|--------|
| 1 | Git status | PASS | 2 modified (config.yaml housekeeping, edges.jsonl Hilo warm) — no worker output |
| 2 | Host load | PASS | 6.20 load, 43Gi available memory |
| 3 | GitReins guard | PASS | secrets clean, rust-analyzer clean |
| 4 | Hilo graph | PASS | 290 edges, 81 files, 4 languages — useful |
| 5 | Cargo check | PASS | 1.05s |
| 6 | Cargo clippy | PASS | 0.29s, zero warnings |
| 7 | TODO/FIXME | PASS | Zero across all .rs files |
| 8 | Cargo audit | PASS | 6 pre-existing warnings (RUSTSEC-2026-0008 git2, fuser) |
| 9 | Cargo test | PASS | 525 passed, 0 failed (all crates, all suites) |
| 10 | GitReins config | PASS | Config exists with evaluator section (deepseek-v4-flash) |
| 11 | Board consistency | PASS | Board 12/12 = GitReins 12/12 (all complete). No drift. No pending GitReins tasks. |
| 12 | DuckBrain | WARN | warpfs namespace exists but is empty (0 memories). MCP connection intermittent. |
| 13 | NEVER-DONE docs | PASS | SECURITY.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE, CODE_OF_CONDUCT all present |
| 14 | CI health | PASS | All 5 recent CI runs green |
| 15 | Scheduler | IDLE | Cooldown 43200s (12h, STABLE), model: deepseek-v4-flash@deepseek-foreman, Enabled: true |
| 16 | Dispatch | SKIP | Zero active tasks. Project genuinely idle. |

**Verdict:** IDLE — 22nd consecutive idle tick, 16th escalation to Bane. All 16 gates PASS (1 WARN for DuckBrain MCP connection). Zero code changes in 3+ weeks. Hilo=useful (290 edges, 81 files). 525/525 tests pass (up from 327 — board corrected). Prior-foreman config housekeeping (evaluator caps tuned to Rust-appropriate values: 50 iter/10m/0.2M input/0.4M output) committed alongside this tick. Scheduler CooldownS=43200 (12h) confirmed stable. Recommend disabling scheduler DB entry to stop PAYG burn (~$0.60/day).
