<!--
  ⚠️  BOARD FORMAT — coding-hermes-model-router v1.3 (2026-07-24)
  All tasks MUST use matrix format: | ID | Task | Pri | Cpx | Deps | Tags | Model | Reasoning | Fallback |
  Before editing this file, load the skill: skill_view(name='coding-hermes-model-router')
  Validate: python3 ~/.hermes/scripts/validate-board-format.py .coding-hermes/tasks.md
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

**Routing Notes:** Board has 0 real tasks — project idle. Scheduler CooldownS=43200 (12h, STABLE — first time in 16 ticks!). 20 idle ticks, 14th escalation to Bane. Cooldown FINALLY persisted across daemon restart. Consider disabling in scheduler DB to stop PAYG burn (~$0.60/day for zero output).

**Execution Order:** NEVER-DONE only.

**Escalation Conditions:** 20 idle ticks, 14th escalation. All checks green. Zero code changes in 2+ weeks. Bane: disable project or accept idle at 12h.

## Completed

| ID | Task | Pri | Cpx | Commit | Model |
|----|------|-----|-----|--------|-------|
| Full build | 12/12 GitReins, 11 crates, 327 tests, semantic search, provenance graph | — | — | v0.2.0 | Various |
