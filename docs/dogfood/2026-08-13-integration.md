# Hilo Integration Report — 2026-08-13

Real-use integration of Hilo (agent-first metadata filesystem, `hilo 0.2.0`)
against a fresh, unfamiliar codebase: `BurntSushi/ripgrep` (111 Rust files).
Run by the coding-hermes dogfood cron. Verdict: 🟡 PROMISING-BUT-ROUGH.

## What was integrated

A working agent-side setup: **CLI graph queries + MCP server + FUSE mount** on a
real repo, exactly as an AI coding agent would use Hilo to orient on a codebase
without reading files.

## The working recipe (verified, with timings)

```bash
# 0. Install (binary: target/release/hilo — NOT hilo-cli; cp to ~/.cargo/bin)
git clone <any-repo> && cd <any-repo>
hilo init                 # 5ms — creates .vfs/, manifest.yaml v2, git hooks
hilo graph warm           # 1.2s on 111 files → 256 edges (82 files, 2 langs)
hilo classify             # 0.18s — role/status xattrs (user.vfs.role, user.vfs.status)
```

Query surface (all verified working, sub-second):

| Query | Result on ripgrep |
|---|---|
| `hilo graph stats` | 164 distinct edges, most connected `pkg:std`, orphans list, top deps |
| `hilo graph impact 'pkg:globset' --max-depth 2` | 3 dependents (correct: 3/3 code importers on disk) |
| `hilo graph search globset --limit 5` | lexical hits with scores + symbols |
| `hilo graph module crates/globset` | 5 files, 11 edges, per-file listing |
| `hilo meta <path>` / `hilo meta --set user.vfs.feature --value demo <path>` | xattr read/write round-trip OK |
| `hilo graph clean` | deletes edges.jsonl + DuckDB cache (rebuild path) |
| `hilo serve --mcp` | 15 `vfs_*` tools over stdio, JSON-RPC 2.0 |
| `hilo mount <dir> --daemon` | returns in 5ms, mount persists, `fusermount -u` clean |

### MCP client (what Claude Code / Hermes would do)

Minimal stdio client: `initialize` → `notifications/initialized` → `tools/list`
(15 tools) → `tools/call`. All calls returned well-formed JSON results;
`vfs_get_metadata` returns `{"backend":"local","path":...,"xattrs":{...}}`.
The protocol layer is genuinely solid — zero protocol friction.

### FUSE (what a shell-based agent would do)

```bash
hilo mount /tmp/mnt --daemon      # instant return
getfattr -n user.vfs.role --only-values /tmp/mnt/crates/globset/src/lib.rs  # → library
fusermount -u /tmp/mnt            # clean unmount
```

## Errors hit and their fixes

| Error | Cause | Fix / workaround |
|---|---|---|
| `hilo graph impact <file>` → "No dependents found" | 100% of edges target `pkg:*` pseudo-nodes; no file→file edges (see diagnostics) | Query the symbol form: `hilo graph impact 'pkg:globset'`. Tracked as **GAP-034** |
| `hilo graph related <file> --direction reverse` → "No incoming edges" | same root cause | `pkg:` form again; GAP-034 |
| MCP `vfs_graph_impact` → `{"dependents":[],"total":0}` | same root cause | GAP-034 |
| `hilo graph untested` lists **all** files incl. test files | classify misses top-level `tests/`/`benches/`; untested doesn't exclude classified tests | None yet — tracked as **GAP-036** |
| `hilo graph understand` → "(no symbols extracted)" for many files | incomplete symbol extraction | None yet — **GAP-037** |
| search/stats show `pkg:{\n    globset` rows | `use crate::{a,b}` truncated at `{` | None yet — **GAP-035/038** |
| `hilo serve` (bare) → "No server mode selected" | `--mcp` is required (fixed GAP-003) | Always `hilo serve --mcp` |
| `./target/release/hilo-cli` → No such file | binary is `hilo` (fixed GAP-001) | Use `./target/release/hilo` |

## What a new user needs that isn't documented

1. **The `pkg:<name>` query form.** The only way file-level questions work today is
   by querying `pkg:<crate>` pseudo-symbols, which nothing in the docs mentions.
   (Fixing this is GAP-034; until then, add a note to cli-reference.md.)
2. **`graph stats` count semantics**: "Total edges" is deduped (164) while
   edges.jsonl has 256 lines — the discrepancy looks like a bug until you know
   DuckDB dedupes multi-provenance pairs.
3. **`understand` takes a natural-language TASK as its positional arg**, not a file
   path (docs are correct; easy to misuse).

## Verdict for maintainers — fix in this order

1. **GAP-034 (P0):** file→file edges or pkg:→file resolution. Without it the
   headline promise ("what would break if I change this?") is unfalsifiable-by-file.
2. **GAP-036 (P1):** test classification → `untested` becomes meaningful.
3. **GAP-035 (P1):** brace-group expansion in the Rust use-parser.
4. GAP-037/038 (P2): symbol extraction depth + stats/search hygiene.

The skeleton is excellent (speed, protocol, FUSE, xattr, MCP). The graph *data*
is what needs the next iteration.
