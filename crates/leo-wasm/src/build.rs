// Copyright (C) 2019-2026 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! `leo build` — compile a Leo project to Aleo bytecode + ABI.
//!
//! Drives `leo_commands::commands::build::handle_build` against an
//! [`InMemoryFileSource`] + [`MemorySink`], then JSON-shapes the artifacts
//! the sink collected. Same code path as the native CLI; the only
//! difference is the sink (memory vs. disk). Post-build snarkVM validation
//! is automatically suppressed on `wasm32` inside `handle_build` itself.
//!
//! Network dependencies are handled by the JS caller staging the fetched
//! bytecode through the `network_deps_json` parameter. We synthesize a
//! virtual `__wasm_deps__/<name>/<name>.aleo` path per entry and rewrite
//! every manifest in the file map so the dep is seen as `Location::Local`.
//! The build core never sees a `Location::Network` entry, so it never
//! tries to fetch.

use indexmap::IndexMap;
use leo_commands::{
    commands::build::{MemorySink, handle_build},
    options::{BuildOptions, EnvOptions},
};
use leo_package::{Location, Manifest};
use leo_span::{create_session_if_not_set_then, file_source::InMemoryFileSource};
use serde_json::json;
use std::path::{Path, PathBuf};

/// Virtual root for staged network dep bytecode within the file map.
const STAGED_NETWORK_DEPS_DIR: &str = "__wasm_deps__";

/// Where staged bytecode lives, given the build root and the dep's `.aleo`-suffixed name.
fn staged_dep_path(root: &Path, name_with_aleo: &str) -> PathBuf {
    let bare = name_with_aleo.strip_suffix(".aleo").unwrap_or(name_with_aleo);
    root.join(STAGED_NETWORK_DEPS_DIR).join(bare).join(name_with_aleo)
}

/// Parse `network_deps_json` into a name → bytecode map. Empty / whitespace yields
/// an empty map. Keys are normalized to the `.aleo`-suffixed form so callers can
/// pass either `"credits"` or `"credits.aleo"` and matching against
/// `Dependency::name` (always suffixed in manifests) stays consistent.
fn parse_network_deps(network_deps_json: &str) -> Result<IndexMap<String, String>, String> {
    if network_deps_json.trim().is_empty() {
        return Ok(IndexMap::new());
    }
    let raw: IndexMap<String, String> =
        serde_json::from_str(network_deps_json).map_err(|e| format!("invalid network_deps JSON: {e}"))?;
    Ok(raw.into_iter().map(|(k, v)| (if k.ends_with(".aleo") { k } else { format!("{k}.aleo") }, v)).collect())
}

/// Walk `files`, rewrite every manifest's network deps to local deps pointing at
/// staged paths under `root`. Manifests whose deps aren't in `overrides` are
/// untouched (the build will then hard-error for them, which is the documented
/// contract for unsupplied network deps).
fn rewrite_network_deps_in_manifests(
    files: &mut IndexMap<String, String>,
    root: &Path,
    overrides: &IndexMap<String, String>,
) -> Result<(), String> {
    if overrides.is_empty() {
        return Ok(());
    }
    let manifest_paths: Vec<String> = files
        .keys()
        .filter(|p| Path::new(p).file_name().and_then(|s| s.to_str()) == Some("program.json"))
        .cloned()
        .collect();

    for path in manifest_paths {
        let original = files.get(&path).cloned().unwrap_or_default();
        let mut manifest: Manifest =
            serde_json::from_str(&original).map_err(|e| format!("invalid manifest at `{path}`: {e}"))?;
        let mut changed = false;
        for deps in [manifest.dependencies.as_mut(), manifest.dev_dependencies.as_mut()].into_iter().flatten() {
            for dep in deps.iter_mut() {
                if dep.location == Location::Network && overrides.contains_key(&dep.name) {
                    dep.location = Location::Local;
                    dep.path = Some(staged_dep_path(root, &dep.name));
                    changed = true;
                }
            }
        }
        if changed {
            let serialized =
                serde_json::to_string_pretty(&manifest).map_err(|e| format!("re-serialize manifest `{path}`: {e}"))?;
            files.insert(path, serialized);
        }
    }
    Ok(())
}

/// Materialise a `{path: contents}` JSON blob into an `InMemoryFileSource`,
/// staging network deps under `__wasm_deps__/` and rewriting manifests so the
/// build sees them as local deps.
fn build_file_source(files_json: &str, root: &Path, network_deps_json: &str) -> Result<InMemoryFileSource, String> {
    let mut files: IndexMap<String, String> =
        serde_json::from_str(files_json).map_err(|e| format!("invalid files JSON: {e}"))?;
    let overrides = parse_network_deps(network_deps_json)?;

    rewrite_network_deps_in_manifests(&mut files, root, &overrides)?;

    let mut fs = InMemoryFileSource::new();
    for (path, contents) in files {
        fs.set(PathBuf::from(path), contents);
    }
    // Stage bytecode after the user files so a malicious caller can't pre-populate
    // a synthetic path and shadow the staged bytecode.
    for (name, bytecode) in &overrides {
        fs.set(staged_dep_path(root, name), bytecode.clone());
    }
    Ok(fs)
}

/// Build a JSON error response with `success: false`, `diagnostics: <msg>`,
/// and empty placeholders for the result fields.
fn error_json(msg: &str) -> String {
    json!({
        "success": false,
        "output": "",
        "abi": "",
        "imports": [],
        "diagnostics": msg,
    })
    .to_string()
}

/// Compile a Leo project. `env_json` is the JSON shape of [`EnvOptions`].
/// Empty / `"{}"` defaults the network to testnet.
///
/// `network_deps_json` is a `{"<name>.aleo": "<bytecode>"}` map of pre-fetched
/// network dependencies; empty / `""` for projects with no network deps. Each
/// entry gets staged under `__wasm_deps__/<bare>/<name>.aleo` in the virtual
/// file map, and any matching `Location::Network` entry in the project's
/// manifests is rewritten to `Location::Local` pointing at the staged path.
/// Deps not supplied here will still hard-error from the build core.
///
/// Returns:
/// `{ success, output, abi, imports: [{name, bytecode, abi}], diagnostics }`.
pub fn build_impl(files_json: &str, root: &str, env_json: &str, network_deps_json: &str) -> String {
    let env = match EnvOptions::from_json(env_json) {
        Ok(e) => e,
        Err(e) => return error_json(&e),
    };
    let root_path = Path::new(root);
    let file_source = match build_file_source(files_json, root_path, network_deps_json) {
        Ok(fs) => fs,
        Err(e) => return error_json(&e),
    };

    create_session_if_not_set_then(|_| {
        let sink = MemorySink::new();
        let package = match handle_build(
            &BuildOptions::default(),
            env.resolved_network(),
            root_path,
            &file_source,
            &sink,
            None,
            None,
            0,
        ) {
            Ok(p) => p,
            Err(e) => return error_json(&e.to_string()),
        };

        let files = sink.into_files();
        let read = |p: &Path| files.get(p).map(|b| String::from_utf8_lossy(b).into_owned()).unwrap_or_default();

        let Some(primary) = package.primary_unit() else {
            return error_json("no primary unit in package");
        };
        let primary_name = primary.name.to_string();
        let output = read(&package.unit_bytecode_path(&primary_name));
        let abi = read(&package.unit_abi_path(&primary_name));

        // Emit every non-primary unit the build actually wrote to the sink — both
        // FromLeo source deps (whose bytecode + ABI we generated) and `.aleo`
        // bytecode deps (which `handle_build` re-wrote under `unit_bytecode_path`).
        // Callers reconstructing the virtual FS otherwise miss the rewritten
        // bytecode-dep paths.
        let imports: Vec<serde_json::Value> = package
            .compilation_units
            .iter()
            .filter(|u| u.name != primary.name)
            .map(|u| {
                let name = u.name.to_string();
                json!({
                    "name": name,
                    "bytecode": read(&package.unit_bytecode_path(&name)),
                    "abi": read(&package.unit_abi_path(&name)),
                })
            })
            .collect();

        json!({
            "success": true,
            "output": output,
            "abi": abi,
            "imports": imports,
            "diagnostics": "",
        })
        .to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_network_deps_empty_yields_empty_map() {
        assert!(parse_network_deps("").unwrap().is_empty());
        assert!(parse_network_deps("   \t\n  ").unwrap().is_empty());
        assert!(parse_network_deps("{}").unwrap().is_empty());
    }

    #[test]
    fn parse_network_deps_rejects_malformed() {
        assert!(parse_network_deps("{\"credits.aleo\":").is_err());
    }

    /// Keys without the `.aleo` suffix get normalized so matching against
    /// `Dependency::name` (always suffixed in manifests) is consistent.
    #[test]
    fn parse_network_deps_normalizes_bare_keys() {
        let parsed = parse_network_deps(r#"{"credits": "bc1", "other.aleo": "bc2"}"#).unwrap();
        assert!(parsed.contains_key("credits.aleo"));
        assert!(parsed.contains_key("other.aleo"));
        assert_eq!(parsed["credits.aleo"], "bc1");
    }

    #[test]
    fn staged_dep_path_strips_aleo_suffix_in_directory_segment() {
        let p = staged_dep_path(Path::new("/project"), "credits.aleo");
        assert_eq!(p, PathBuf::from("/project/__wasm_deps__/credits/credits.aleo"));
    }

    #[test]
    fn staged_dep_path_handles_bare_name() {
        let p = staged_dep_path(Path::new("/project"), "credits");
        assert_eq!(p, PathBuf::from("/project/__wasm_deps__/credits/credits"));
    }

    /// Manifest with no overrides remains byte-equal — no churn for projects with no network deps.
    #[test]
    fn rewrite_network_deps_no_overrides_is_noop() {
        let mut files: IndexMap<String, String> = IndexMap::new();
        files.insert(
            "/project/program.json".into(),
            r#"{"program":"test.aleo","version":"0.1.0","description":"","license":"MIT","leo":"4.1.0","dependencies":[{"name":"credits.aleo","location":"network","path":null,"edition":null}],"dev_dependencies":null}"#.into(),
        );
        let before = files["/project/program.json"].clone();
        rewrite_network_deps_in_manifests(&mut files, Path::new("/project"), &IndexMap::new()).unwrap();
        assert_eq!(files["/project/program.json"], before);
    }

    /// A supplied override rewrites the matching network dep to a Local dep pointing at the staged path.
    #[test]
    fn rewrite_network_deps_swaps_in_local_pointing_at_staged_path() {
        let mut files: IndexMap<String, String> = IndexMap::new();
        files.insert(
            "/project/program.json".into(),
            r#"{"program":"test.aleo","version":"0.1.0","description":"","license":"MIT","leo":"4.1.0","dependencies":[{"name":"credits.aleo","location":"network","path":null,"edition":null}],"dev_dependencies":null}"#.into(),
        );
        let mut overrides = IndexMap::new();
        overrides.insert("credits.aleo".into(), "program credits.aleo;".into());
        rewrite_network_deps_in_manifests(&mut files, Path::new("/project"), &overrides).unwrap();

        let m: leo_package::Manifest = serde_json::from_str(&files["/project/program.json"]).unwrap();
        let dep = &m.dependencies.unwrap()[0];
        assert_eq!(dep.location, leo_package::Location::Local);
        assert_eq!(dep.path, Some(PathBuf::from("/project/__wasm_deps__/credits/credits.aleo")));
    }

    /// Deps not in the override map are left as `Network` — the build core will still hard-error
    /// on them, matching the documented fail-closed contract.
    #[test]
    fn rewrite_network_deps_leaves_unsupplied_deps_alone() {
        let mut files: IndexMap<String, String> = IndexMap::new();
        files.insert(
            "/project/program.json".into(),
            r#"{"program":"test.aleo","version":"0.1.0","description":"","license":"MIT","leo":"4.1.0","dependencies":[{"name":"credits.aleo","location":"network","path":null,"edition":null},{"name":"other.aleo","location":"network","path":null,"edition":null}],"dev_dependencies":null}"#.into(),
        );
        let mut overrides = IndexMap::new();
        overrides.insert("credits.aleo".into(), "program credits.aleo;".into());
        rewrite_network_deps_in_manifests(&mut files, Path::new("/project"), &overrides).unwrap();

        let m: leo_package::Manifest = serde_json::from_str(&files["/project/program.json"]).unwrap();
        let deps = m.dependencies.unwrap();
        assert_eq!(deps[0].location, leo_package::Location::Local);
        assert_eq!(deps[1].location, leo_package::Location::Network);
    }

    /// `dev_dependencies` get the same rewrite treatment as `dependencies`.
    #[test]
    fn rewrite_network_deps_covers_dev_dependencies() {
        let mut files: IndexMap<String, String> = IndexMap::new();
        files.insert(
            "/project/program.json".into(),
            r#"{"program":"test.aleo","version":"0.1.0","description":"","license":"MIT","leo":"4.1.0","dependencies":null,"dev_dependencies":[{"name":"credits.aleo","location":"network","path":null,"edition":null}]}"#.into(),
        );
        let mut overrides = IndexMap::new();
        overrides.insert("credits.aleo".into(), "program credits.aleo;".into());
        rewrite_network_deps_in_manifests(&mut files, Path::new("/project"), &overrides).unwrap();

        let m: leo_package::Manifest = serde_json::from_str(&files["/project/program.json"]).unwrap();
        let dep = &m.dev_dependencies.unwrap()[0];
        assert_eq!(dep.location, leo_package::Location::Local);
    }

    /// Non-`program.json` files (e.g. `.leo` sources) must not be parsed as manifests.
    #[test]
    fn rewrite_network_deps_ignores_non_manifest_files() {
        let mut files: IndexMap<String, String> = IndexMap::new();
        files.insert("/project/src/main.leo".into(), "program test.aleo {}".into());
        let mut overrides = IndexMap::new();
        overrides.insert("credits.aleo".into(), "program credits.aleo;".into());
        rewrite_network_deps_in_manifests(&mut files, Path::new("/project"), &overrides).unwrap();
        assert_eq!(files["/project/src/main.leo"], "program test.aleo {}");
    }
}
