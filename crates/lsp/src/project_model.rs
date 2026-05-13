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

use leo_package::{CompilationUnit, MANIFEST_FILENAME, Manifest, PackageKind, ProgramData, SOURCE_DIRECTORY};
use leo_span::{Symbol, create_session_if_not_set_then};
use lsp_types::Uri;
use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

/// Resolved package information for one Leo source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectContext {
    /// Canonical package root containing `program.json`.
    pub package_root: Arc<PathBuf>,
    /// Canonical path to the owning manifest.
    pub manifest_path: Arc<PathBuf>,
    /// Canonical path to the package source directory.
    pub source_directory: Arc<PathBuf>,
    /// Canonical entry file used for program or library analysis.
    pub entry_file: Arc<PathBuf>,
    /// Manifest program name used when constructing compiler state.
    pub program_name: Arc<str>,
    /// Whether the package entry point is a deployable program or a library.
    pub kind: ProjectKind,
    /// Content fingerprint of `manifest_path` used to invalidate cached contexts.
    manifest_revision: u64,
}

/// Package entry kind relevant to LSP parsing and compiler analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectKind {
    /// A package with `src/main.leo`.
    Program,
    /// A package with `src/lib.leo`.
    Library,
}

/// Cache for lightweight Leo package-root resolution.
///
/// The cache is keyed by the concrete document path and stores only verified
/// package roots. Misses are recomputed so a file can become managed later in
/// the same editor session without restarting the server.
#[derive(Debug, Default)]
pub struct ProjectModel {
    cache: HashMap<PathBuf, Arc<ProjectContext>>,
}

impl ProjectModel {
    /// Resolve the native file path and owning Leo project context for a document URI.
    pub fn resolve_document_context(&mut self, uri: &Uri) -> (Option<Arc<PathBuf>>, Option<Arc<ProjectContext>>) {
        let path = uri_to_file_path(uri).map(normalize_document_path);
        let project = path.as_deref().and_then(|path| self.resolve_project_context(path));
        (path.map(Arc::new), project)
    }

    /// Resolve the nearest Leo project context for the provided document path.
    pub fn resolve_project_context(&mut self, path: &Path) -> Option<Arc<ProjectContext>> {
        if let Some(cached) = self.cache.get(path) {
            // Revalidate cached hits so deleting, moving, or editing the
            // manifest does not leave the server pinned to stale package data.
            if manifest_revision(cached.manifest_path.as_ref()) == Some(cached.manifest_revision)
                && cached.entry_file.is_file()
            {
                return Some(Arc::clone(cached));
            }

            self.cache.remove(path);
        }

        // Cache the nearest manifest ancestor for this exact document path so
        // reopen and change notifications can reuse the same lookup result.
        let context = build_project_context(find_package_root(path)?)?;
        let context = Arc::new(context);
        self.cache.insert(path.to_path_buf(), Arc::clone(&context));
        Some(context)
    }
}

/// Convert an LSP file URI into a local filesystem path.
pub(crate) fn uri_to_file_path(uri: &Uri) -> Option<PathBuf> {
    if uri.scheme().map(|scheme| scheme.as_str()) != Some("file") {
        return None;
    }

    // `Uri::path()` remains percent-encoded, so decode it before converting to
    // a native path on either Unix or Windows.
    let decoded_path = uri.path().as_estr().decode().into_string().ok()?;
    let decoded_path = decoded_path.into_owned();

    #[cfg(target_os = "windows")]
    {
        let trimmed = decoded_path.strip_prefix('/').unwrap_or(decoded_path.as_str());
        Some(PathBuf::from(trimmed))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Some(PathBuf::from(decoded_path))
    }
}

/// Convert a native filesystem path into an LSP `file:` URI.
///
/// The helper keeps URI construction centralized so editor responses do not
/// depend on hand-built `file://{path}` strings. It canonicalizes paths when
/// possible, preserves unresolved paths when necessary, and percent-encodes
/// bytes that are not safe in a URI path.
pub(crate) fn path_to_file_uri(path: &Path) -> Option<Uri> {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    #[cfg(target_os = "windows")]
    let raw = {
        let display = path.display().to_string();
        let display = display.strip_prefix(r"\\?\").unwrap_or(display.as_str());
        format!("/{}", display.replace('\\', "/"))
    };

    #[cfg(not(target_os = "windows"))]
    let raw = path.display().to_string();

    format!("file://{}", percent_encode_path(&raw)).parse().ok()
}

/// Percent-encode a native path string for the path component of a `file:` URI.
fn percent_encode_path(path: &str) -> String {
    let mut encoded = String::with_capacity(path.len());
    for byte in path.bytes() {
        if matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'.' | b'-' | b'_' | b'~' | b':') {
            encoded.push(byte as char);
        } else {
            use std::fmt::Write;
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}

/// Find the nearest package root that owns the given document path.
fn find_package_root(path: &Path) -> Option<PathBuf> {
    let start = if path.is_dir() { Some(path) } else { path.parent() }?;

    // Walk upward from the document location so nested Leo packages resolve to
    // the closest owning manifest rather than the workspace root.
    for ancestor in start.ancestors() {
        let manifest_path = ancestor.join(MANIFEST_FILENAME);
        if manifest_path.is_file() {
            // Canonicalize once here so downstream comparisons and snapshots use
            // a stable package-root path.
            return ancestor.canonicalize().ok();
        }
    }

    None
}

/// Build the full project context for a canonical package root.
///
/// This resolves the manifest, source directory, entry file, and package kind
/// in one place so the rest of the LSP can treat project discovery as a single
/// cached lookup.
fn build_project_context(package_root: PathBuf) -> Option<ProjectContext> {
    create_session_if_not_set_then(|_| {
        // Package resolution interns symbols through the compiler/span crates,
        // so project discovery must run inside a Leo session just like the
        // worker-side compiler analysis path.
        let manifest_path = package_root.join(MANIFEST_FILENAME);
        let manifest_revision = manifest_revision(&manifest_path)?;
        let manifest = Manifest::read_from_file(&manifest_path).ok()?;
        let source_directory = package_root.join(SOURCE_DIRECTORY);
        source_directory.read_dir().ok()?;

        let compilation_unit =
            CompilationUnit::from_package_path(Symbol::intern(&manifest.program), &package_root).ok()?;
        let (entry_file, kind) = match compilation_unit.data {
            ProgramData::SourcePath { source, .. } => {
                let kind = match compilation_unit.kind {
                    PackageKind::Program => ProjectKind::Program,
                    PackageKind::Library => ProjectKind::Library,
                    PackageKind::Test => return None,
                };
                (normalize_document_path(source), kind)
            }
            ProgramData::Bytecode(_) => return None,
        };

        Some(ProjectContext {
            package_root: Arc::new(package_root),
            manifest_path: Arc::new(normalize_document_path(manifest_path)),
            source_directory: Arc::new(normalize_document_path(source_directory)),
            entry_file: Arc::new(entry_file),
            program_name: Arc::from(manifest.program),
            kind,
            manifest_revision,
        })
    })
}

/// Hash the manifest contents used to invalidate cached contexts.
fn manifest_revision(path: &Path) -> Option<u64> {
    let mut hasher = DefaultHasher::new();
    std::fs::read(path).ok()?.hash(&mut hasher);
    Some(hasher.finish())
}

/// Canonicalize a document path when possible while preserving unresolved paths.
fn normalize_document_path(path: PathBuf) -> PathBuf {
    // Keep the original path when canonicalization fails so transient editor
    // buffers or not-yet-created files can still participate in lookups.
    path.canonicalize().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::{ProjectKind, ProjectModel};
    use lsp_types::Uri;
    use std::{fs, path::Path};
    use tempfile::tempdir;

    /// Build a test `file:` URI from a native path.
    fn file_uri(path: &Path) -> Uri {
        #[cfg(target_os = "windows")]
        let path = format!("/{}", path.display()).replace('\\', "/");

        #[cfg(not(target_os = "windows"))]
        let path = path.display().to_string();

        format!("file://{path}").parse().expect("file uri")
    }

    /// Verifies project discovery chooses the nearest manifest ancestor.
    #[test]
    fn resolves_nearest_manifest_ancestor() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let nested = package_root.join("src").join("deep");
        fs::create_dir_all(&nested).expect("create nested directories");
        fs::write(
            package_root.join("program.json"),
            r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
        )
        .expect("write manifest");
        fs::write(package_root.join("src").join("main.leo"), "program demo.aleo {}").expect("write main");

        let file_uri = file_uri(&nested.join("main.leo"));
        let mut projects = ProjectModel::default();
        let canonical_root = package_root.canonicalize().expect("canonical package root");

        let (file_path, resolved) = projects.resolve_document_context(&file_uri);
        let resolved = resolved.expect("project context");
        assert_eq!(file_path.expect("file path").as_ref(), &nested.join("main.leo"));
        assert_eq!(resolved.package_root.as_ref(), &canonical_root);
        assert_eq!(resolved.kind, ProjectKind::Program);
    }

    /// Verifies non-file documents remain unmanaged.
    #[test]
    fn non_file_uris_are_unmanaged() {
        let mut projects = ProjectModel::default();
        let uri: Uri = "untitled:main.leo".parse().expect("untitled uri");

        let (file_path, project) = projects.resolve_document_context(&uri);
        assert!(file_path.is_none());
        assert!(project.is_none());
    }

    /// Verifies a previous miss can become managed after a manifest appears.
    #[test]
    fn discovers_manifest_added_after_initial_miss() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let source_dir = package_root.join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(source_dir.join("main.leo"), "program demo.aleo {}").expect("write main");

        let file_uri = file_uri(&source_dir.join("main.leo"));
        let mut projects = ProjectModel::default();

        assert!(projects.resolve_document_context(&file_uri).1.is_none());

        fs::write(
            package_root.join("program.json"),
            r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
        )
        .expect("write manifest");

        let resolved = projects.resolve_document_context(&file_uri).1.expect("package root after manifest add");
        assert_eq!(resolved.package_root.as_ref(), &package_root.canonicalize().expect("canonical package root"));
    }

    /// Verifies cached project roots are invalidated after manifest removal.
    #[test]
    fn invalidates_cached_root_after_manifest_removal() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let source_dir = package_root.join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(source_dir.join("main.leo"), "program demo.aleo {}").expect("write main");
        fs::write(
            package_root.join("program.json"),
            r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
        )
        .expect("write manifest");

        let file_uri = file_uri(&source_dir.join("main.leo"));
        let mut projects = ProjectModel::default();

        assert!(projects.resolve_document_context(&file_uri).1.is_some());

        fs::remove_file(package_root.join("program.json")).expect("remove manifest");

        assert!(projects.resolve_document_context(&file_uri).1.is_none());
    }

    /// Verifies library packages resolve to library entrypoints.
    #[test]
    fn resolves_library_entrypoints() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let source_dir = package_root.join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(source_dir.join("lib.leo"), "fn helper() -> u32 { 1u32 }").expect("write lib");
        fs::write(
            package_root.join("program.json"),
            r#"{ "program": "math_lib", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
        )
        .expect("write manifest");

        let file_uri = file_uri(&source_dir.join("lib.leo"));
        let mut projects = ProjectModel::default();

        let resolved = projects.resolve_document_context(&file_uri).1.expect("library context");
        assert_eq!(resolved.kind, ProjectKind::Library);
        assert_eq!(resolved.program_name.as_ref(), "math_lib");
    }

    /// Verifies manifest content changes refresh cached project context.
    #[test]
    fn refreshes_cached_context_after_manifest_content_change() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let source_dir = package_root.join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(source_dir.join("main.leo"), "program demo.aleo {}").expect("write main");
        let manifest_path = package_root.join("program.json");
        fs::write(
            &manifest_path,
            r#"{ "program": "demo.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
        )
        .expect("write manifest");

        let file_uri = file_uri(&source_dir.join("main.leo"));
        let mut projects = ProjectModel::default();
        let first = projects.resolve_document_context(&file_uri).1.expect("initial project context");
        assert_eq!(first.program_name.as_ref(), "demo.aleo");

        std::thread::sleep(std::time::Duration::from_millis(20));
        fs::write(
            &manifest_path,
            r#"{ "program": "updated.aleo", "version": "0.1.0", "description": "", "license": "MIT", "leo": "4.0.0" }"#,
        )
        .expect("rewrite manifest");

        let refreshed = projects.resolve_document_context(&file_uri).1.expect("refreshed project context");
        assert_eq!(refreshed.program_name.as_ref(), "updated.aleo");
    }

    /// Verifies same-size manifest rewrites produce a new revision.
    #[test]
    fn manifest_revision_changes_on_same_size_rewrite() {
        let tempdir = tempdir().expect("tempdir");
        let manifest_path = tempdir.path().join("program.json");
        fs::write(&manifest_path, "aaaaaaaa").expect("write manifest");
        let first = super::manifest_revision(&manifest_path).expect("initial revision");

        fs::write(&manifest_path, "bbbbbbbb").expect("rewrite manifest");
        let second = super::manifest_revision(&manifest_path).expect("updated revision");

        assert_ne!(first, second);
    }
}
