// Copyright (C) 2019-2025 Provable Inc.
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

//! The source map provides an address space for positions in spans
//! that is global across the source files that are compiled together.
//! The source files are organized in a sequence,
//! with the positions of each source following the ones of the previous source
//! in the address space of positions
//! (except for the first source, which starts at the beginning of the address space).
//! This way, any place in any source is identified by a single position
//! within the address space covered by the sequence of sources;
//! the source file is determined from the position.

use crate::span::Span;

use std::{
    cell::RefCell,
    fmt,
    fs,
    io,
    path::{Path, PathBuf},
    rc::Rc,
};

/// The source map containing all recorded sources,
/// methods to register new ones,
/// and methods to query about spans in relation to recorded sources.
#[derive(Default)]
pub struct SourceMap {
    /// The actual source map data.
    inner: RefCell<SourceMapInner>,
}

/// Actual data of the source map.
/// We use this setup for purposes of interior mutability.
#[derive(Default)]
struct SourceMapInner {
    /// The address space below this value is currently used by the files in the source map.
    used_address_space: u32,

    /// All the source files recorded thus far.
    ///
    /// The list is append-only with mappings from the start byte position
    /// for fast lookup from a `Span` to its `SourceFile`.
    source_files: Vec<Rc<SourceFile>>,
}

impl SourceMap {
    /// Loads the given `path` and returns a `SourceFile` for it.
    pub fn load_file(&self, path: &Path) -> io::Result<Rc<SourceFile>> {
        Ok(self.new_source(&fs::read_to_string(path)?, FileName::Real(path.to_owned())))
    }

    /// Registers `source` under the given file `name`, returning a `SourceFile` back.
    pub fn new_source(&self, source: &str, name: FileName) -> Rc<SourceFile> {
        let len = u32::try_from(source.len()).unwrap();
        let mut inner = self.inner.borrow_mut();
        let start_pos = inner.try_allocate_address_space(len).unwrap();
        let source_file = Rc::new(SourceFile::new(name, source.to_owned(), start_pos));
        inner.source_files.push(source_file.clone());
        source_file
    }

    /// Find the index for the source file containing `pos`.
    fn find_source_file_index(&self, pos: u32) -> Option<usize> {
        self.inner
            .borrow()
            .source_files
            .binary_search_by_key(&pos, |file| file.absolute_start)
            .map_or_else(|p| p.checked_sub(1), Some)
    }

    /// Find the source file containing `pos`.
    pub fn find_source_file(&self, pos: u32) -> Option<Rc<SourceFile>> {
        Some(self.inner.borrow().source_files[self.find_source_file_index(pos)?].clone())
    }

    /// Returns the source contents that is spanned by `span`.
    pub fn contents_of_span(&self, span: Span) -> Option<String> {
        let source_file1 = self.find_source_file(span.lo)?;
        let source_file2 = self.find_source_file(span.hi)?;
        assert_eq!(source_file1.absolute_start, source_file2.absolute_start);
        Some(source_file1.contents_of_span(span).to_string())
    }
}

impl SourceMapInner {
    /// Attempt reserving address space for `size` number of bytes.
    fn try_allocate_address_space(&mut self, size: u32) -> Option<u32> {
        let current = self.used_address_space;
        // By adding one, we can distinguish between files, even when they are empty.
        self.used_address_space = current.checked_add(size)?.checked_add(1)?;
        Some(current)
    }
}

/// A file name.
///
/// This is either a wrapper around `PathBuf`,
/// or a custom string description.
#[derive(Clone)]
pub enum FileName {
    /// A real file.
    Real(PathBuf),
    /// Any sort of description for a source.
    Custom(String),
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Real(x) if is_color() => x.display().fmt(f),
            Self::Real(_) => Ok(()),
            Self::Custom(x) => f.write_str(x),
        }
    }
}

/// Is the env var `NOCOLOR` not enabled?
pub fn is_color() -> bool {
    std::env::var("NOCOLOR").unwrap_or_default().trim().is_empty()
}

/// A single source in the [`SourceMap`].
pub struct SourceFile {
    /// The name of the file that the source came from.
    pub name: FileName,
    /// The complete source code.
    pub src: String,
    /// The start position of this source in the `SourceMap`.
    pub absolute_start: u32,
    /// The end position of this source in the `SourceMap`.
    pub absolute_end: u32,
}

impl SourceFile {
    /// Creates a new `SourceFile`.
    fn new(name: FileName, src: String, absolute_start: u32) -> Self {
        let absolute_end = absolute_start + src.len() as u32;
        Self { name, src, absolute_start, absolute_end }
    }

    /// Converts an absolute offset to a file-relative offset
    pub fn relative_offset(&self, absolute_offset: u32) -> u32 {
        assert!(self.absolute_start <= absolute_offset);
        assert!(absolute_offset <= self.absolute_end);
        absolute_offset - self.absolute_start
    }

    /// Returns contents of a `span` assumed to be within the given file.
    pub fn contents_of_span(&self, span: Span) -> &str {
        let start = self.relative_offset(span.lo);
        let end = self.relative_offset(span.hi);
        &self.src[start as usize..end as usize]
    }

    pub fn line_col(&self, absolute_offset: u32) -> (u32, u32) {
        let relative_offset = self.relative_offset(absolute_offset);
        let mut current_offset = 0u32;

        for (i, line) in self.src.split('\n').enumerate() {
            let end_of_line = current_offset + line.len() as u32;
            if relative_offset <= end_of_line {
                let chars = self.src[current_offset as usize..relative_offset as usize].chars().count();
                return (i as u32, chars as u32);
            }
            current_offset = end_of_line + 1;
        }

        panic!("Can't happen.");
    }

    pub fn line_contents(&self, span: Span) -> LineContents<'_> {
        let start = self.relative_offset(span.lo) as usize;
        let end = self.relative_offset(span.hi) as usize;

        let line_start = self.src[..=start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = self.src[end..].find('\n').map(|x| x + end).unwrap_or(self.src.len());

        LineContents {
            line: self.src[..line_start].lines().count(),
            contents: &self.src[line_start..line_end],
            start: start.saturating_sub(line_start),
            end: end.saturating_sub(line_start),
        }
    }
}

pub struct LineContents<'a> {
    pub contents: &'a str,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl fmt::Display for LineContents<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const INDENT: &str = "    ";

        let mut current_underline = String::new();
        let mut line = self.line;
        let mut line_beginning = true;
        let mut underline_started = false;

        writeln!(f, "{INDENT} |")?;

        for (i, c) in self.contents.chars().enumerate() {
            if line_beginning {
                write!(
                    f,
                    "{line:width$} | ",
                    // Report lines starting from 1.
                    line = line + 1,
                    width = INDENT.len()
                )?;
            }
            if c == '\n' {
                writeln!(f)?;
                // Output the underline, without trailing whitespace.
                let underline = current_underline.trim_end();
                if !underline.is_empty() {
                    writeln!(f, "{INDENT} | {underline}")?;
                }
                underline_started = false;
                current_underline.clear();
                line += 1;
                line_beginning = true;
            } else {
                line_beginning = false;
                if c != '\r' {
                    write!(f, "{c}")?;
                    if self.start <= i && i < self.end && (underline_started || !c.is_whitespace()) {
                        underline_started = true;
                        current_underline.push('^');
                    } else {
                        current_underline.push(' ');
                    }
                }
            }
        }

        // If the text didn't end in a newline, we may still
        // need to output an underline.
        let underline = current_underline.trim_end();
        if !underline.is_empty() {
            writeln!(f, "\n{INDENT} | {underline}")?;
        }

        Ok(())
    }
}

/// File / Line / Column information on a `BytePos`.
pub struct LineCol {
    /// Information on the original source.
    pub source_file: Rc<SourceFile>,
    /// The line number.
    pub line: u32,
    /// The column offset into the line.
    pub col: u32,
}
