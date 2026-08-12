# hilo-cli — Command-Line Interface

The `hilo` binary — entrypoint for all Hilo operations. Built with clap.

**Crate:** `hilo-cli`  
**Binary:** `hilo`

## Commands

| Command | Description |
|---------|-------------|
| `hilo init` | Initialize a Hilo project — creates `.vfs/`, installs git hooks |
| `hilo meta <path> [--set <attr> --value <value>]` | Read (all) or set xattr metadata on a file — no per-key read/list flags |
| `hilo graph related <path> [--direction reverse] [--relation imports]` | Query forward/reverse dependency edges |
| `hilo graph impact <path>` | Transitive blast-radius analysis |
| `hilo graph stats` | Aggregate graph statistics |
| `hilo graph warm [--language <lang>] [--changed]` | Pre-parse all files into DuckDB cache |
| `hilo graph module <name>` | All edges for a module |
| `hilo graph untested` | Files with no test coverage edges |
| `hilo graph understand <TASK> [--budget <N>]` | Multi-resolution harmonic context output for a natural-language task |
| `hilo graph search <QUERY> [--limit <N>]` | Deterministic semantic code search (TF-IDF + BM25) |
| `hilo graph rule-list` | List all rules defined in the manifest |
| `hilo graph rule-check <NAME>` | Execute a named rule query against the dependency graph |
| `hilo serve --mcp` | Start MCP server (stdio) — `--mcp` required (only implemented server mode); rate limit read from manifest `performance.rate_limit_rps` |
| `hilo backend mount --type s3 --bucket <BUCKET> --at <PATH> [--prefix <PREFIX>] [--region <REGION>]` | Mount a virtual backend (S3, git, local) at a virtual path |
| `hilo backend list` | List all mounted backends |
| `hilo mount <mount-point> [--triggers] [--allow-other]` | Mount FUSE filesystem |
| `hilo workspace mount <MOUNT_POINT> [--manifest <PATH>]` | Mount all repos and backends from the workspace manifest (default `.vfs/manifest.yaml`) |
| `hilo workspace unmount <MOUNT_POINT>` | Unmount a workspace |
| `hilo classify [--dry-run]` | Auto-classify all files (role/status/feature metadata) |
| `hilo plugin list` | List loaded WASM plugins |
| `hilo plugin load <path>` | Load a plugin |

## Usage Examples

```bash
# Initialize
hilo init

# Query graph
hilo graph related src/main.go
hilo graph impact src/auth/mod.rs
hilo graph warm --language go --language rust
hilo graph stats

# Metadata
hilo meta --set user.vfs.feature --value auth-module src/auth.rs
# Read (all xattrs): hilo meta src/auth.rs — no per-key --read/--list flags

# Classify
hilo classify --dry-run
hilo classify

# Backends
hilo backend mount --type s3 --bucket my-bucket --region us-east-1 --at /s3
hilo backend list

# Workspaces
hilo workspace mount /mnt/hilo
hilo workspace unmount /mnt/hilo

# Mount
hilo mount /mnt/hilo --triggers

# Plugins
hilo plugin load ./my-plugin.wasm
hilo plugin list
```
