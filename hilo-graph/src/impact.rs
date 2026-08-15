//! Transitive impact analysis — find all files that depend on a given file, directly or transitively.

use std::collections::{HashSet, VecDeque};

use duckdb::{params, Connection};
use serde::Serialize;

use crate::error::GraphResult;
use crate::resolution::PkgResolver;

/// A single file in the impact chain.
#[derive(Debug, Clone, Serialize)]
pub struct ImpactFile {
    /// The file path.
    pub path: String,
    /// The relation type from the edge that connects this file to its dependent.
    pub relation: String,
    /// Distance from the start file (1 = direct dependent, N = N-hop dependent).
    pub depth: u32,
    /// How the edge was discovered (provenance string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    /// Confidence weight (0.0 – 1.0) of the edge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// Result of an impact analysis.
#[derive(Debug, Clone, Serialize)]
pub struct ImpactResult {
    pub files: Vec<ImpactFile>,
}

/// Collect query rows into the BFS result set (visited-dedup, depth-tagged).
fn collect(
    rows: impl Iterator<Item = duckdb::Result<(String, String, Option<String>, Option<f64>)>>,
    results: &mut Vec<ImpactFile>,
    visited: &mut HashSet<String>,
    queue: &mut VecDeque<(String, u32)>,
    depth: u32,
) -> GraphResult<()> {
    for row in rows {
        let (from, rel, prov, conf) = row?;
        if visited.insert(from.clone()) {
            results.push(ImpactFile {
                path: from.clone(),
                relation: rel,
                depth: depth + 1,
                provenance: prov,
                confidence: conf,
            });
            queue.push_back((from, depth + 1));
        }
    }
    Ok(())
}

/// Compute transitive impact: find all files that depend on `start_path`,
/// directly or transitively, up to `max_depth` hops.
///
/// The DuckDB `edges` table has columns `"from"` (source file), `"to"` (dependency),
/// `rel` (relation type). Impact analysis finds files whose `"from"` appears as
/// a dependent of `start_path` or its transitive dependents.
///
/// Uses BFS with a visited set to protect against circular imports.
/// Returns files ordered by discovery (BFS order) — direct dependents first,
/// then 2-hop, etc.
pub fn compute_impact(
    conn: &Connection,
    start_path: &str,
    max_depth: u32,
) -> GraphResult<Vec<ImpactFile>> {
    if max_depth == 0 {
        return Ok(Vec::new());
    }

    let mut results: Vec<ImpactFile> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start_path.to_string());

    let mut queue: VecDeque<(String, u32)> = VecDeque::new();
    queue.push_back((start_path.to_string(), 0));

    let mut stmt =
        conn.prepare(r#"SELECT "from", rel, provenance, confidence FROM edges WHERE "to" = ?"#)?;
    let mut resolver = PkgResolver::new();

    while let Some((path, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        // Exact match (local edges).
        collect(
            stmt.query_map(params![path.clone()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<f64>>(3)?,
                ))
            })?,
            &mut results,
            &mut visited,
            &mut queue,
            depth,
        )?;

        // GAP-034: file-level resolution — a file query must also match
        // dependents that target its crate's `pkg:<name>` node, because the
        // parser emits `pkg:` edges (not file→file edges). Symbol nodes
        // (`pkg:...`/`sys:...`) are not files and resolve to None.
        if let Some(pkg) = resolver.pkg_node(&path) {
            collect(
                stmt.query_map(params![pkg.clone()], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, Option<f64>>(3)?,
                    ))
                })?,
                &mut results,
                &mut visited,
                &mut queue,
                depth,
            )?;
        }
    }

    Ok(results)
}

/// Compute transitive impact with external cross-repo edge support.
///
/// When `include_external` is `true`, the BFS also follows `external:<repo>:<path>`
/// edges by matching `to LIKE '%:' || path`.  This allows impact analysis
/// to traverse across repository boundaries in a multi-repo workspace.
pub fn compute_impact_with_external(
    conn: &Connection,
    start_path: &str,
    max_depth: u32,
    include_external: bool,
) -> GraphResult<Vec<ImpactFile>> {
    if max_depth == 0 {
        return Ok(Vec::new());
    }

    let mut results: Vec<ImpactFile> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start_path.to_string());

    let mut queue: VecDeque<(String, u32)> = VecDeque::new();
    queue.push_back((start_path.to_string(), 0));

    let mut stmt =
        conn.prepare(r#"SELECT "from", rel, provenance, confidence FROM edges WHERE "to" = ?"#)?;
    let mut ext_stmt: Option<duckdb::Statement> = None;
    let mut resolver = PkgResolver::new();

    if include_external {
        // Match edges where `to` ends with `:path` (the external edge format
        // is `external:repo-name:path`).
        ext_stmt = Some(conn.prepare(
            r#"SELECT "from", rel, provenance, confidence FROM edges WHERE "to" LIKE '%:' || ?"#,
        )?);
    }

    while let Some((path, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        // Exact match (local edges).
        collect(
            stmt.query_map(params![path.clone()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<f64>>(3)?,
                ))
            })?,
            &mut results,
            &mut visited,
            &mut queue,
            depth,
        )?;

        // GAP-034: file-level resolution — match dependents that target the
        // file's crate `pkg:<name>` node (the parser emits pkg: edges, not
        // file→file edges). Symbol nodes resolve to None.
        if let Some(pkg) = resolver.pkg_node(&path) {
            collect(
                stmt.query_map(params![pkg.clone()], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, Option<f64>>(3)?,
                    ))
                })?,
                &mut results,
                &mut visited,
                &mut queue,
                depth,
            )?;
        }

        // External-edge match (cross-repo).
        // Convert `repo/path/to/file` → `repo:path/to/file` by replacing
        // only the first `/` with `:` to match the `external:repo:path` format.
        if let Some(ref mut estmt) = ext_stmt {
            let ext_path = path.replacen('/', ":", 1);
            let rows = estmt.query_map(params![ext_path], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, Option<f64>>(3)?,
                ))
            })?;
            for row in rows {
                let (from, rel, prov, conf) = row?;
                if visited.insert(from.clone()) {
                    results.push(ImpactFile {
                        path: from.clone(),
                        relation: rel,
                        depth: depth + 1,
                        provenance: prov,
                        confidence: conf,
                    });
                    queue.push_back((from, depth + 1));
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::insert_edges_into;
    use crate::parser::{Language, Parser};

    /// Build a two-crate temp workspace:
    /// crates/a (lib) ← crates/b (bin, `use a::Glob;`).
    /// The parser emits (b/src/main.rs → pkg:a); nothing points at the file
    /// crates/a/src/lib.rs directly — resolution must bridge the gap.
    fn build_two_crate_workspace() -> (tempfile::TempDir, String, String) {
        let dir = tempfile::tempdir().unwrap();
        let write = |rel: &str, content: &str| {
            let path = dir.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, content).unwrap();
            path.to_string_lossy().into_owned()
        };
        write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"crates/a\", \"crates/b\"]\n",
        );
        write(
            "crates/a/Cargo.toml",
            "[package]\nname = \"a\"\nversion = \"0.1.0\"\n",
        );
        let lib = write("crates/a/src/lib.rs", "pub struct Glob;\n");
        write(
            "crates/b/Cargo.toml",
            "[package]\nname = \"b\"\nversion = \"0.1.0\"\n",
        );
        let main = write("crates/b/src/main.rs", "use a::Glob;\nfn main() {}\n");
        (dir, lib, main)
    }

    #[test]
    fn impact_on_crate_root_resolves_pkg_dependents() {
        let (_dir, lib, main) = build_two_crate_workspace();

        let conn = duckdb::Connection::open_in_memory().unwrap();
        insert_edges_into(&conn, &[]).unwrap();

        // Parse both files exactly like ensure_parsed/warm does.
        let parse = |path: &str| {
            let source = std::fs::read_to_string(path).unwrap();
            let mut parser = Parser::for_language(Language::Rust).unwrap();
            parser.parse_imports(path, &source).unwrap()
        };
        insert_edges_into(&conn, &parse(&lib)).unwrap();
        insert_edges_into(&conn, &parse(&main)).unwrap();

        // Sanity: pkg: query works (the old happy path).
        let via_pkg = compute_impact(&conn, "pkg:a", 10).unwrap();
        assert!(
            via_pkg.iter().any(|f| f.path == main),
            "pkg:a must be found by main.rs, got: {via_pkg:?}"
        );

        // The GAP-034 fix: file-level query resolves to the crate pkg node.
        let via_file = compute_impact(&conn, &lib, 10).unwrap();
        assert!(
            via_file.iter().any(|f| f.path == main),
            "impact on crate root file must resolve to pkg:a dependents, got: {via_file:?}"
        );
        assert_eq!(via_file[0].depth, 1);
    }

    #[test]
    fn impact_unknown_file_without_manifest_resolves_none() {
        // A file not under any Cargo.toml must not blow up and keeps the
        // exact-match behavior (empty results are fine).
        let dir = tempfile::tempdir().unwrap();
        let orphan = dir.path().join("src/orphan.rs");
        std::fs::create_dir_all(orphan.parent().unwrap()).unwrap();
        std::fs::write(&orphan, "fn x() {}\n").unwrap();
        let orphan = orphan.to_string_lossy().into_owned();

        let conn = duckdb::Connection::open_in_memory().unwrap();
        insert_edges_into(&conn, &[]).unwrap();
        let results = compute_impact(&conn, &orphan, 10).unwrap();
        assert!(results.is_empty());
    }
}
