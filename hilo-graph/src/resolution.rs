//! Query-time resolution layer — map file paths to the `pkg:` graph nodes
//! they belong to (GAP-034).
//!
//! The parser emits edges that target `pkg:<name>` pseudo-nodes (crate
//! names) rather than file paths, so a file-level query like
//! `hilo graph impact crates/globset/src/lib.rs` finds nothing unless the
//! file is resolved to its crate's `pkg:` node first. This module walks up
//! from a file to the nearest `Cargo.toml` with a `[package] name` and
//! derives the `pkg:<name>` node the file belongs to.
//!
//! Resolution is query-time only: the canonical `edges.jsonl` and the
//! DuckDB cache are untouched. Files that belong to no package (or are not
//! files at all — `pkg:`/`sys:`/`external:` symbol nodes) resolve to `None`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolves file paths to their `pkg:` nodes, caching results per query.
///
/// A single resolver instance should be created per top-level query (impact
/// BFS, related lookup) so repeated visits to the same path cost one
/// filesystem walk at most.
#[derive(Default)]
pub struct PkgResolver {
    cache: HashMap<PathBuf, Option<String>>,
}

impl PkgResolver {
    /// Create an empty resolver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve `path` to its `pkg:<crate>` node, if the path is a file that
    /// belongs to a Cargo package.
    ///
    /// Symbol-node paths (`pkg:...`, `sys:...`, `external:...`) are not
    /// files and always resolve to `None` — they must never trigger a
    /// filesystem walk (a path like `pkg:globset` would otherwise be
    /// interpreted relative to the current directory).
    pub fn pkg_node(&mut self, path: &str) -> Option<String> {
        self.crate_name(path).map(|name| format!("pkg:{name}"))
    }

    /// Resolve `path` to its Cargo package name, if any.
    pub fn crate_name(&mut self, path: &str) -> Option<String> {
        if path.starts_with("pkg:") || path.starts_with("sys:") || path.starts_with("external:") {
            return None;
        }
        let p = PathBuf::from(path);
        if let Some(hit) = self.cache.get(&p) {
            return hit.clone();
        }
        let name = crate_name_for_file(&p);
        self.cache.insert(p, name.clone());
        name
    }
}

/// Walk up from `file` to the nearest `Cargo.toml` with a `[package] name`,
/// and return that name.
///
/// Workspace-root manifests (no `[package]` section) are skipped so a crate
/// nested under a workspace still resolves to its own package name. Returns
/// `None` when no manifest with a package name is found before the
/// filesystem root.
pub fn crate_name_for_file(file: &Path) -> Option<String> {
    let mut dir = file.parent()?;
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.is_file() {
            if let Some(name) = package_name(&manifest) {
                return Some(name);
            }
            // Workspace root or manifest without [package]: keep walking up.
        }
        dir = dir.parent()?;
    }
}

/// Extract the `name` value from the `[package]` section of a Cargo.toml.
fn package_name(manifest: &Path) -> Option<String> {
    let text = std::fs::read_to_string(manifest).ok()?;
    let mut in_package = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line.starts_with("[package]");
            continue;
        }
        if !in_package {
            continue;
        }
        let rest = line.strip_prefix("name")?.trim_start();
        let rest = rest.strip_prefix('=')?.trim_start();
        let start = rest.find('"')?;
        let after = &rest[start + 1..];
        let end = after.find('"')?;
        let value = after[..end].trim();
        if value.is_empty() {
            return None;
        }
        return Some(value.to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, rel: &str, content: &str) -> String {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn resolves_nested_crate_src_file() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "crates/globset/Cargo.toml",
            "[package]\nname = \"globset\"\nversion = \"0.4.0\"\n",
        );
        let lib = write(
            dir.path(),
            "crates/globset/src/lib.rs",
            "pub struct Glob;\n",
        );
        assert_eq!(
            crate_name_for_file(Path::new(&lib)).as_deref(),
            Some("globset")
        );
        let mut resolver = PkgResolver::new();
        assert_eq!(resolver.pkg_node(&lib).as_deref(), Some("pkg:globset"));
    }

    #[test]
    fn skips_workspace_root_manifest() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "Cargo.toml",
            "[workspace]\nmembers = [\"crates/a\"]\n",
        );
        write(
            dir.path(),
            "crates/a/Cargo.toml",
            "[package]\nname = \"a\"\nversion = \"0.1.0\"\n",
        );
        let main = write(dir.path(), "crates/a/src/main.rs", "fn main() {}\n");
        assert_eq!(crate_name_for_file(Path::new(&main)).as_deref(), Some("a"));
    }

    #[test]
    fn symbol_nodes_never_resolve() {
        let mut resolver = PkgResolver::new();
        assert_eq!(resolver.pkg_node("pkg:globset"), None);
        assert_eq!(resolver.pkg_node("sys:std"), None);
        assert_eq!(resolver.pkg_node("external:repo:path"), None);
    }

    #[test]
    fn file_without_manifest_resolves_none() {
        let dir = tempfile::tempdir().unwrap();
        let orphan = write(dir.path(), "src/orphan.rs", "fn x() {}\n");
        let mut resolver = PkgResolver::new();
        assert_eq!(resolver.pkg_node(&orphan), None);
    }

    #[test]
    fn package_name_ignores_comments_and_other_sections() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "Cargo.toml",
            "[workspace]\nname = \"not-the-package\"\n\n[package]\nname = \"real\" # the crate\nversion = \"1.0.0\"\n",
        );
        let src = write(dir.path(), "src/lib.rs", "pub fn f() {}\n");
        assert_eq!(
            crate_name_for_file(Path::new(&src)).as_deref(),
            Some("real")
        );
    }
}
