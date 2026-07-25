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

**Assumptions:** Rust project — `cargo check`, `cargo fmt`, `cargo test --workspace` all PASS (327 tests). `cargo audit`: 6 pre-existing warnings. CI all green (latest 3 runs). Zero source TODOs. GitReins: 12/12 complete.

**Routing Notes:** Board has 0 real tasks — project idle. Scheduler CooldownS=43200 (12h, STABLE). 21 idle ticks, 15th escalation to Bane. Cooldown persisted across daemon restart (confirmed 7 ticks). Consider disabling in scheduler DB to stop PAYG burn (~$0.60/day for zero output).

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

**Verdict:** IDLE — 21st consecutive idle tick, 15th escalation to Bane. All 12 gates PASS. Zero code changes in 2+ weeks. Hilo=useful (201 edges, 81 files). Suggest disabling scheduler DB entry to stop PAYG burn.
