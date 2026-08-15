# hilo-ffi — UniFFI Language Bindings

The `hilo-ffi` crate (11th workspace member) defines the UniFFI interface
for Hilo and generates language bindings for **Go, Python, Kotlin, and
Swift** — letting non-Rust applications use Hilo's metadata and graph
primitives through a native SDK.

## Interface

The `.udl` file (`src/hilo.udl`) is the **source of truth**. It defines 8
functions:

| Function | Description |
|---|---|
| `vfs_get_metadata(path, key)` | Get a `user.vfs.*` xattr value |
| `vfs_set_metadata(path, key, value)` | Set a `user.vfs.*` xattr value |
| `vfs_graph_related(path)` | Get graph edges related to a file |
| `vfs_graph_impact(path, max_depth)` | Transitive dependents via BFS |
| `vfs_graph_stats()` | Graph summary statistics |
| `vfs_resolve_backend(path)` | Backend info for a virtual path |
| `vfs_rule_check(rule_name)` | Run a DuckDB rule query |
| `vfs_list_directory(path)` | List virtual directory entries |

Supporting dictionaries (`MetadataResult`, `SetMetadataResult`,
`GraphEdge`, `GraphRelatedResult`, `GraphImpactEntry`,
`GraphImpactResult`, `GraphStats`, `BackendInfo`, `RuleCheckResult`,
`DirectoryListing`) and the `HiloError` enum (`InvalidInput`, `NotFound`,
`BackendUnavailable`, `InternalError`) are defined in the same UDL.

## Generating Bindings

```bash
# Go
uniffi-bindgen generate src/hilo.udl --language go --out-dir hilo-go/vfs/

# Python
uniffi-bindgen generate src/hilo.udl --language python --out-dir hilo/

# Kotlin
uniffi-bindgen generate src/hilo.udl --language kotlin --out-dir hilo-kotlin/

# Swift
uniffi-bindgen generate src/hilo.udl --language swift --out-dir Hilo/
```

Generated code is NOT committed — the `.udl` is the source of truth.

## Output Targets

| Language | Output directory | Package/Module |
|---|---|---|
| Go | `hilo-go/vfs/` | `vfs` |
| Python | `hilo/` | `hilo` wheel |
| Kotlin | `hilo-kotlin/` | `hilo-kotlin` |
| Swift | `Hilo/` | `Hilo` |

## Build

```bash
cargo build -p hilo_ffi
```

The build script (`build.rs`) auto-generates Rust scaffolding from
`hilo.udl`.
