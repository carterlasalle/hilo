# CLI Reference

## `hilo init`

Initialize Hilo in the current directory. Creates `.vfs/` with inventory
files and a default manifest.

```bash
hilo init
```

## `hilo meta`

Read and write extended attributes on files.

```bash
# Read all Hilo xattrs
hilo meta src/main.rs

# Set a specific attribute
hilo meta --set user.vfs.role --value entrypoint src/main.rs

# Read (prints all xattrs — no per-key read flag)
hilo meta src/main.rs
```

## `hilo graph`

### `warm`

Walk the directory tree, parse all source files with tree-sitter, and
build the dependency graph. Writes to `.vfs/graph/edges.jsonl` and
`.vfs/graph/graph.db`.

```bash
hilo graph warm

# With cross-repo workspace edges
hilo graph warm --workspace
```

Supported languages (26): Go, Python, TypeScript, Rust, JavaScript,
Java, C, C++, Ruby, C#, Kotlin, PHP, Swift, Elixir, Haskell, Erlang,
Scala, Zig, Lua, Dart, Clojure, OCaml, R, Julia, Elm, Nim.
Directories skipped: `target/`, `node_modules/`, `vendor/`,
`__pycache__/`, `.venv/`.

### `stats`

Aggregate statistics about the dependency graph.

```bash
hilo graph stats

# Output:
# Total edges: 2252
# Unique source files: 716
# Unique dependencies: 531
# Top dependencies:
#   sys:gtest/gtest.h: 349
#   sys:metacall/metacall.h: 175
```

### `related`

Find files related to a given path through the dependency graph.

```bash
# Forward: what does this file import?
hilo graph related src/main.rs

# Filter by relation type
hilo graph related src/main.rs --relation imports

# Reverse: what imports this file?
hilo graph related sys:some-header.h --direction reverse

# Reverse with relation filter
hilo graph related src/login.go --direction reverse --relation tested_by
```

### `impact`

Find all files that depend on a given file, directly or transitively.

```bash
# Direct dependents only
hilo graph impact sys:metacall/metacall.h --max-depth 1

# Full transitive closure (default: 10)
hilo graph impact sys:gtest/gtest.h --max-depth 10

# JSON output
hilo graph impact sys:metacall/metacall.h --format json
```

### `understand`

Multi-resolution harmonic context output for a natural-language task.

```bash
hilo graph understand "how does plugin execution get sandboxed"
```

Token budget override (default: 6000):

```bash
hilo graph understand "how does plugin execution get sandboxed" --budget 12000
```

### `search`

Deterministic semantic code search (TF-IDF + BM25).

```bash
# Top 20 matches (default)
hilo graph search "rate limiter"

# Custom result limit
hilo graph search "rate limiter" --limit 50
```

### `module`

Per-module statistics and test coverage.

```bash
hilo graph module hilo-graph/src
```

### `untested`

List source files with no test coverage.

```bash
hilo graph untested
```

### `rule-list`

List all rules defined in the manifest.

```bash
hilo graph rule-list
```

### `rule-check`

Execute a named rule query against the dependency graph.

```bash
hilo graph rule-check stale-files
```

## `hilo classify`

Auto-tag every source file with `user.vfs.role` and `user.vfs.status`
using tree-sitter AST queries. No LLM required.

```bash
# Dry run — show what would be tagged
hilo classify --dry-run

# Apply tags
hilo classify

# Verbose output (per-file)
hilo classify --verbose
```

Roles detected: `entrypoint`, `library`, `test`, `script`, `example`,
`config`, `unknown`.

Statuses detected: `stable`, `beta`, `unstable`, `deprecated`, `unknown`.

## `hilo mount`

Mount the current directory as a FUSE filesystem with xattr passthrough.

```bash
mkdir /mnt/vfs
hilo mount /mnt/vfs

# With triggers (auto-reparse on file changes)
hilo mount /mnt/vfs --triggers

# Allow other users to access
hilo mount /mnt/vfs --allow-other
```

## `hilo serve`

Start the MCP server for agent integration.

```bash
# Stdio transport (for Claude Desktop, Hermes)
hilo serve --mcp
```

## `hilo backend`

Manage virtual backends (S3, git, remote, local).

### `mount`

Mount a virtual backend.

```bash
hilo backend mount --type s3 --bucket my-bucket --prefix data --at /s3

# Explicit region (default: us-east-1)
hilo backend mount --type s3 --bucket my-bucket --at /s3 --region eu-west-1
```

### `list`

List all mounted backends.

```bash
hilo backend list
```

## `hilo workspace`

Manage multi-repo workspace mounts.

### `mount`

Mount all repos and backends from the manifest.

```bash
hilo workspace mount /mnt/hilo
```

### `unmount`

Unmount a workspace.

```bash
hilo workspace unmount /mnt/hilo
```

## `hilo plugin`

Load and manage wasm plugins.

### `load`

Load a .wasm plugin and register it in the runtime.

```bash
hilo plugin load ./my-plugin.wasm
```

### `list`

List plugins discovered in `.vfs/plugins/`.

```bash
hilo plugin list
```
