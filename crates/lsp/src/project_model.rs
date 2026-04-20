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

use leo_package::MANIFEST_FILENAME;
use lsp_types::Uri;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Cache for lightweight Leo package-root resolution.
///
/// The cache is keyed by the concrete document path and stores only verified
/// package roots. Misses are recomputed so a file can become managed later in
/// the same editor session without restarting the server.
#[derive(Debug, Default)]
pub struct ProjectModel {
    cache: HashMap<PathBuf, Arc<PathBuf>>,
}

impl ProjectModel {
    /// Resolve the nearest Leo package root for the provided document URI.
    pub fn resolve_package_root(&mut self, uri: &Uri) -> Option<Arc<PathBuf>> {
        let path = uri_to_file_path(uri)?;

        if let Some(cached) = self.cache.get(&path) {
            // Revalidate cached hits so deleting or moving a manifest does not
            // leave the server pinned to a stale package root.
            if cached.join(MANIFEST_FILENAME).is_file() {
                return Some(Arc::clone(cached));
            }

            self.cache.remove(&path);
        }

        // Cache the nearest manifest ancestor for this exact document path so
        // reopen and change notifications can reuse the same lookup result.
        let root = find_package_root(&path).map(Arc::new)?;
        self.cache.insert(path.to_path_buf(), Arc::clone(&root));
        Some(root)
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

#[cfg(test)]
mod tests {
    use super::ProjectModel;
    use lsp_types::Uri;
    use std::{fs, path::Path};
    use tempfile::tempdir;

    fn file_uri(path: &Path) -> Uri {
        #[cfg(target_os = "windows")]
        let path = format!("/{}", path.display()).replace('\\', "/");

        #[cfg(not(target_os = "windows"))]
        let path = path.display().to_string();

        format!("file://{path}").parse().expect("file uri")
    }

    #[test]
    fn resolves_nearest_manifest_ancestor() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let nested = package_root.join("src").join("deep");
        fs::create_dir_all(&nested).expect("create nested directories");
        fs::write(package_root.join("program.json"), "{}").expect("write manifest");

        let file_uri = file_uri(&nested.join("main.leo"));
        let mut projects = ProjectModel::default();
        let canonical_root = package_root.canonicalize().expect("canonical package root");

        let resolved = projects.resolve_package_root(&file_uri).expect("package root");
        assert_eq!(resolved.as_ref(), &canonical_root);
    }

    #[test]
    fn non_file_uris_are_unmanaged() {
        let mut projects = ProjectModel::default();
        let uri: Uri = "untitled:main.leo".parse().expect("untitled uri");

        assert!(projects.resolve_package_root(&uri).is_none());
    }

    #[test]
    fn discovers_manifest_added_after_initial_miss() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let source_dir = package_root.join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");

        let file_uri = file_uri(&source_dir.join("main.leo"));
        let mut projects = ProjectModel::default();

        assert!(projects.resolve_package_root(&file_uri).is_none());

        fs::write(package_root.join("program.json"), "{}").expect("write manifest");

        let resolved = projects.resolve_package_root(&file_uri).expect("package root after manifest add");
        assert_eq!(resolved.as_ref(), &package_root.canonicalize().expect("canonical package root"));
    }

    #[test]
    fn invalidates_cached_root_after_manifest_removal() {
        let tempdir = tempdir().expect("tempdir");
        let package_root = tempdir.path().join("workspace").join("package");
        let source_dir = package_root.join("src");
        fs::create_dir_all(&source_dir).expect("create source dir");
        fs::write(package_root.join("program.json"), "{}").expect("write manifest");

        let file_uri = file_uri(&source_dir.join("main.leo"));
        let mut projects = ProjectModel::default();

        assert!(projects.resolve_package_root(&file_uri).is_some());

        fs::remove_file(package_root.join("program.json")).expect("remove manifest");

        assert!(projects.resolve_package_root(&file_uri).is_none());
    }
}
