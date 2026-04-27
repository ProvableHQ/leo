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

use std::{
    ffi::OsStr,
    fs,
    io,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;

/// Abstraction over where the compiler reads source files from.
///
/// The default implementation [`DiskFileSource`] reads from the real filesystem.
/// Alternative implementations enable compilation and formatting from in-memory
/// buffers without requiring disk I/O.
///
/// # Path contract
///
/// Callers must provide consistent paths. Path normalization is the caller's
/// responsibility.
///
/// - `exclude` in [`FileSource::list_leo_files`] is compared by exact path
///   equality, so it must match the listed path exactly.
/// - [`InMemoryFileSource::read_file`] performs exact key lookup.
///
/// # Ordering
///
/// [`FileSource::list_leo_files`] must return paths in deterministic, sorted
/// order to ensure reproducible module ordering.
pub trait FileSource {
    /// Read the contents of a file at the given path.
    fn read_file(&self, path: &Path) -> io::Result<String>;

    /// List all `.leo` files under `dir`, excluding `exclude`.
    fn list_leo_files(&self, dir: &Path, exclude: &Path) -> io::Result<Vec<PathBuf>>;
}

/// Reads source files from the real filesystem.
pub struct DiskFileSource;

impl FileSource for DiskFileSource {
    fn read_file(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn list_leo_files(&self, dir: &Path, exclude: &Path) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        walk_dir_recursive(dir, exclude, &mut files)?;
        files.sort();
        Ok(files)
    }
}

/// Recursively walks `dir`, collecting `.leo` files and propagating I/O errors.
fn walk_dir_recursive(dir: &Path, exclude: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir_recursive(&path, exclude, files)?;
        } else if path != exclude && path.extension() == Some(OsStr::new("leo")) {
            files.push(path);
        }
    }

    Ok(())
}

/// Reads source files from in-memory buffers.
#[derive(Default)]
pub struct InMemoryFileSource {
    files: IndexMap<PathBuf, String>,
}

impl InMemoryFileSource {
    /// Creates a new empty in-memory file source.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces the contents of a file.
    pub fn set(&mut self, path: PathBuf, contents: String) {
        self.files.insert(path, contents);
    }
}

impl FileSource for InMemoryFileSource {
    fn read_file(&self, path: &Path) -> io::Result<String> {
        self.files.get(path).cloned().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, path.display().to_string()))
    }

    fn list_leo_files(&self, dir: &Path, exclude: &Path) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::with_capacity(self.files.len());
        for path in self.files.keys() {
            if path.starts_with(dir) && path != exclude && path.extension() == Some(OsStr::new("leo")) {
                files.push(path.clone());
            }
        }

        files.sort();
        Ok(files)
    }
}

/// Reads one overlay file from memory and falls back to another file source for everything else.
pub struct OverlayFileSource<'a, F> {
    overlay_path: PathBuf,
    overlay_contents: String,
    fallback: &'a F,
}

impl<'a, F> OverlayFileSource<'a, F> {
    /// Creates a new overlay file source.
    pub fn new(overlay_path: PathBuf, overlay_contents: String, fallback: &'a F) -> Self {
        Self { overlay_path, overlay_contents, fallback }
    }
}

impl<F: FileSource> FileSource for OverlayFileSource<'_, F> {
    fn read_file(&self, path: &Path) -> io::Result<String> {
        if path == self.overlay_path { Ok(self.overlay_contents.clone()) } else { self.fallback.read_file(path) }
    }

    fn list_leo_files(&self, dir: &Path, exclude: &Path) -> io::Result<Vec<PathBuf>> {
        let mut files = self.fallback.list_leo_files(dir, exclude)?;

        if self.overlay_path.starts_with(dir)
            && self.overlay_path.extension() == Some(OsStr::new("leo"))
            && self.overlay_path != exclude
            && !files.iter().any(|path| path == &self.overlay_path)
        {
            files.push(self.overlay_path.clone());
            files.sort();
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::{DiskFileSource, FileSource, InMemoryFileSource, OverlayFileSource};

    use std::{
        env,
        fs,
        io,
        path::{Path, PathBuf},
        process,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn unique_temp_dir() -> PathBuf {
        let nanos =
            SystemTime::now().duration_since(UNIX_EPOCH).expect("system clock should be after unix epoch").as_nanos();
        env::temp_dir().join(format!("leo_file_source_{}_{}", process::id(), nanos))
    }

    #[test]
    fn in_memory_read_file() {
        let mut source = InMemoryFileSource::new();
        source.set(PathBuf::from("/src/main.leo"), "program test.aleo { }".into());

        let content = source.read_file(Path::new("/src/main.leo")).unwrap();
        assert_eq!(content, "program test.aleo { }");
    }

    #[test]
    fn in_memory_read_file_not_found() {
        let source = InMemoryFileSource::new();

        let err = source.read_file(Path::new("/nonexistent.leo")).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn in_memory_list_leo_files() {
        let mut source = InMemoryFileSource::new();
        source.set(PathBuf::from("/src/main.leo"), String::new());
        source.set(PathBuf::from("/src/utils.leo"), String::new());
        source.set(PathBuf::from("/src/alpha.leo"), String::new());
        source.set(PathBuf::from("/src/data.json"), String::new());
        source.set(PathBuf::from("/other/lib.leo"), String::new());

        let files = source.list_leo_files(Path::new("/src"), Path::new("/src/main.leo")).unwrap();
        assert_eq!(files, vec![PathBuf::from("/src/alpha.leo"), PathBuf::from("/src/utils.leo")]);
    }

    #[test]
    fn disk_read_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

        let content = DiskFileSource.read_file(&path).unwrap();
        assert!(content.contains("leo-span"));
    }

    #[test]
    fn disk_read_file_not_found() {
        let err = DiskFileSource.read_file(Path::new("/nonexistent_path_12345.leo")).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn disk_list_leo_files() {
        let tmp = unique_temp_dir();
        let nested = tmp.join("nested");
        let excluded = tmp.join("excluded.leo");
        let result = (|| -> io::Result<Vec<PathBuf>> {
            fs::create_dir_all(&nested)?;
            fs::write(tmp.join("b.leo"), "")?;
            fs::write(tmp.join("a.leo"), "")?;
            fs::write(&excluded, "")?;
            fs::write(tmp.join("not_leo.txt"), "")?;
            fs::write(nested.join("nested.leo"), "")?;

            DiskFileSource.list_leo_files(&tmp, &excluded)
        })();

        let _ = fs::remove_dir_all(&tmp);

        let files = result.unwrap();
        assert_eq!(files, vec![tmp.join("a.leo"), tmp.join("b.leo"), nested.join("nested.leo")]);
    }

    #[test]
    fn disk_list_leo_files_propagates_errors() {
        let err = DiskFileSource.list_leo_files(Path::new("/nonexistent_dir_12345"), Path::new("")).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn in_memory_deterministic_ordering() {
        let mut source = InMemoryFileSource::new();
        source.set(PathBuf::from("/src/z.leo"), String::new());
        source.set(PathBuf::from("/src/m.leo"), String::new());
        source.set(PathBuf::from("/src/a.leo"), String::new());

        let files = source.list_leo_files(Path::new("/src"), Path::new("/src/none.leo")).unwrap();
        assert_eq!(files, vec![PathBuf::from("/src/a.leo"), PathBuf::from("/src/m.leo"), PathBuf::from("/src/z.leo")]);
    }

    #[test]
    fn overlay_source_reads_overlay_before_fallback() {
        let mut fallback = InMemoryFileSource::new();
        fallback.set(PathBuf::from("/src/main.leo"), "disk".into());
        let overlay = OverlayFileSource::new(PathBuf::from("/src/main.leo"), "memory".into(), &fallback);

        let content = overlay.read_file(Path::new("/src/main.leo")).unwrap();
        assert_eq!(content, "memory");
    }

    #[test]
    fn overlay_source_merges_overlay_file_into_listings() {
        let mut fallback = InMemoryFileSource::new();
        fallback.set(PathBuf::from("/src/utils.leo"), String::new());
        let overlay = OverlayFileSource::new(PathBuf::from("/src/main.leo"), String::new(), &fallback);

        let files = overlay.list_leo_files(Path::new("/src"), Path::new("/src/none.leo")).unwrap();
        assert_eq!(files, vec![PathBuf::from("/src/main.leo"), PathBuf::from("/src/utils.leo")]);
    }
}
