// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::span::{BytePos, CharPos, Pos, Span};
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
    fn find_source_file_index(&self, pos: BytePos) -> Option<usize> {
        self.inner
            .borrow()
            .source_files
            .binary_search_by_key(&pos, |file| file.start_pos)
            .map_or_else(|p| p.checked_sub(1), Some)
    }

    /// Find the source file containing `pos`.
    fn find_source_file(&self, pos: BytePos) -> Option<Rc<SourceFile>> {
        Some(self.inner.borrow().source_files[self.find_source_file_index(pos)?].clone())
    }

    /// Finds line column info about a given `pos`.
    fn find_line_col(&self, pos: BytePos) -> Option<LineCol> {
        let source_file = self.find_source_file(pos)?;
        let (line, col) = source_file.lookup_file_pos(pos);
        Some(LineCol { source_file, line, col })
    }

    /// Retrives the location (source file, line, col) on the given span.
    pub fn span_to_location(&self, sp: Span) -> Option<SpanLocation> {
        let lo = self.find_line_col(sp.lo)?;
        let hi = self.find_line_col(sp.hi)?;
        Some(SpanLocation {
            source_file: lo.source_file,
            line_start: lo.line,
            line_stop: hi.line,
            col_start: lo.col.to_usize() + 1,
            col_stop: hi.col.to_usize() + 1,
        })
    }

    /// Returns a displayable representation of the `span` as a string.
    pub fn span_to_string(&self, span: Span) -> String {
        let loc = match self.span_to_location(span) {
            None => return "no-location".to_string(),
            Some(l) => l,
        };

        if loc.line_start == loc.line_stop {
            format!("{}:{}-{}", loc.line_start, loc.col_start, loc.col_stop)
        } else {
            format!("{}:{}-{}:{}", loc.line_start, loc.col_start, loc.line_stop, loc.col_stop)
        }
    }

    /// Returns the source contents that is spanned by `span`.
    pub fn contents_of_span(&self, span: Span) -> Option<String> {
        let begin = self.find_source_file(span.lo)?;
        let end = self.find_source_file(span.hi)?;
        assert_eq!(begin.start_pos, end.start_pos);
        Some(begin.contents_of_span(span))
    }

    /// Returns the source contents of the lines that `span` is within.
    ///
    /// That is, if the span refers to `x = 4` in the source code:
    ///
    /// > ```text
    /// > // Line 1
    /// > let x
    /// >     = 4;
    /// > // Line 4
    /// > ```
    ///
    /// then the contents on lines 2 and 3 are returned.
    pub fn line_contents_of_span(&self, span: Span) -> Option<String> {
        let begin = self.find_source_file(span.lo)?;
        let end = self.find_source_file(span.hi)?;
        assert_eq!(begin.start_pos, end.start_pos);

        let idx_lo = begin.lookup_line(span.lo).unwrap_or(0);
        let idx_hi = begin.lookup_line(span.hi).unwrap_or(0) + 1;
        let lo_line_pos = begin.lines[idx_lo];
        let hi_line_pos = if idx_hi < begin.lines.len() { begin.lines[idx_hi] } else { begin.end_pos };
        Some(begin.contents_of_span(Span::new(lo_line_pos, hi_line_pos)))
    }
}

impl SourceMapInner {
    /// Attempt reserving address space for `size` number of bytes.
    fn try_allocate_address_space(&mut self, size: u32) -> Option<BytePos> {
        let current = self.used_address_space;
        // By adding one, we can distinguish between files, even when they are empty.
        self.used_address_space = current.checked_add(size)?.checked_add(1)?;
        Some(BytePos(current))
    }
}

/// A file name.
///
/// For now it's simply a wrapper around `PathBuf`,
/// but may become more complicated in the future.
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
            Self::Real(x) if is_not_test_framework() => x.display().fmt(f),
            Self::Real(_) => Ok(()),
            Self::Custom(x) => f.write_str(x),
        }
    }
}

/// Is the env var `LEO_TESTFRAMEWORK` not enabled?
pub fn is_not_test_framework() -> bool {
    std::env::var("LEO_TESTFRAMEWORK").unwrap_or_default().trim().to_owned().is_empty()
}

/// A single source in the [`SourceMap`].
pub struct SourceFile {
    /// The name of the file that the source came from.
    pub name: FileName,
    /// The complete source code.
    pub src: String,
    /// The start position of this source in the `SourceMap`.
    pub start_pos: BytePos,
    /// The end position of this source in the `SourceMap`.
    pub end_pos: BytePos,
    /// Locations of line beginnings in the source code.
    lines: Vec<BytePos>,
    /// Locations of multi-byte characters in the source code.
    multibyte_chars: Vec<MultiByteChar>,
}

impl SourceFile {
    /// Creates a new `SourceMap` given the file `name`,
    /// source contents, and the `start_pos`ition.
    ///
    /// This position is used for analysis purposes.
    fn new(name: FileName, mut src: String, start_pos: BytePos) -> Self {
        normalize_src(&mut src);
        let end_pos = start_pos + BytePos::from_usize(src.len());
        let (lines, multibyte_chars) = analyze_source_file(&src, start_pos);
        Self { name, src, start_pos, end_pos, lines, multibyte_chars }
    }

    /// Converts an absolute `BytePos` to a `CharPos` relative to the `SourceFile`.
    fn bytepos_to_file_charpos(&self, bpos: BytePos) -> CharPos {
        // The number of extra bytes due to multibyte chars in the `SourceFile`.
        let mut total_extra_bytes = 0;

        for mbc in self.multibyte_chars.iter() {
            if mbc.pos < bpos {
                // Every character is at least one byte, so we only
                // count the actual extra bytes.
                total_extra_bytes += mbc.bytes as u32 - 1;
                // We should never see a byte position in the middle of a
                // character.
                assert!(bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32);
            } else {
                break;
            }
        }

        assert!(self.start_pos.to_u32() + total_extra_bytes <= bpos.to_u32());
        CharPos(bpos.to_usize() - self.start_pos.to_usize() - total_extra_bytes as usize)
    }

    /// Finds the line containing the given position. The return value is the
    /// index into the `lines` array of this `SourceFile`, not the 1-based line
    /// number. If the source_file is empty or the position is located before the
    /// first line, `None` is returned.
    fn lookup_line(&self, pos: BytePos) -> Option<usize> {
        match self.lines.binary_search(&pos) {
            Ok(idx) => Some(idx),
            Err(0) => None,
            Err(idx) => Some(idx - 1),
        }
    }

    /// Looks up the file's (1-based) line number and (0-based `CharPos`) column offset, for a
    /// given `BytePos`.
    fn lookup_file_pos(&self, pos: BytePos) -> (usize, CharPos) {
        let chpos = self.bytepos_to_file_charpos(pos);
        match self.lookup_line(pos) {
            Some(a) => {
                let line = a + 1; // Line numbers start at 1
                let linebpos = self.lines[a];
                let linechpos = self.bytepos_to_file_charpos(linebpos);
                let col = chpos - linechpos;
                assert!(chpos >= linechpos);
                (line, col)
            }
            None => (0, chpos),
        }
    }

    /// Returns contents of a `span` assumed to be within the given file.
    fn contents_of_span(&self, span: Span) -> String {
        let begin_pos = self.bytepos_to_file_charpos(span.lo).to_usize();
        let end_pos = self.bytepos_to_file_charpos(span.hi).to_usize();
        String::from_utf8_lossy(&self.src.as_bytes()[begin_pos..end_pos]).into_owned()
    }
}

/// Detailed information on a `Span`.
pub struct SpanLocation {
    pub source_file: Rc<SourceFile>,
    pub line_start: usize,
    pub line_stop: usize,
    pub col_start: usize,
    pub col_stop: usize,
}

impl SpanLocation {
    /// Returns a dummy location.
    pub fn dummy() -> Self {
        let dummy = "<dummy>".to_owned();
        let span = Span::dummy();
        Self {
            source_file: Rc::new(SourceFile {
                name: FileName::Custom(dummy.clone()),
                src: dummy,
                start_pos: span.lo,
                end_pos: span.hi,
                lines: Vec::new(),
                multibyte_chars: Vec::new(),
            }),
            line_start: 0,
            line_stop: 0,
            col_start: 0,
            col_stop: 0,
        }
    }
}

/// File / Line / Column information on a `BytePos`.
pub struct LineCol {
    /// Information on the original source.
    pub source_file: Rc<SourceFile>,
    /// The 1-based line number.
    pub line: usize,
    /// The (0-based) column offset into the line.
    pub col: CharPos,
}

/// Normalizes the source code and records the normalizations.
fn normalize_src(src: &mut String) {
    remove_bom(src);
    normalize_newlines(src);
}

/// Removes UTF-8 BOM, if any.
fn remove_bom(src: &mut String) {
    if src.starts_with('\u{feff}') {
        src.drain(..3);
    }
}

/// Replaces `\r\n` with `\n` in-place in `src`.
///
/// Returns error if there's a lone `\r` in the string.
fn normalize_newlines(src: &mut String) {
    if !src.as_bytes().contains(&b'\r') {
        return;
    }

    // We replace `\r\n` with `\n` in-place, which doesn't break utf-8 encoding.
    // While we *can* call `as_mut_vec` and do surgery on the live string
    // directly, let's rather steal the contents of `src`. This makes the code
    // safe even if a panic occurs.

    let mut buf = std::mem::take(src).into_bytes();
    let mut gap_len = 0;
    let mut tail = buf.as_mut_slice();
    loop {
        let idx = match find_crlf(&tail[gap_len..]) {
            None => tail.len(),
            Some(idx) => idx + gap_len,
        };
        tail.copy_within(gap_len..idx, 0);
        tail = &mut tail[idx - gap_len..];
        if tail.len() == gap_len {
            break;
        }
        gap_len += 1;
    }

    // Account for removed `\r`.
    // After `buf.truncate(..)`, `buf` is guaranteed to contain utf-8 again.
    let new_len = buf.len() - gap_len;
    buf.truncate(new_len);
    *src = String::from_utf8(buf).unwrap();

    fn find_crlf(src: &[u8]) -> Option<usize> {
        let mut search_idx = 0;
        while let Some(idx) = find_cr(&src[search_idx..]) {
            if src[search_idx..].get(idx + 1) != Some(&b'\n') {
                search_idx += idx + 1;
                continue;
            }
            return Some(search_idx + idx);
        }
        None
    }

    fn find_cr(src: &[u8]) -> Option<usize> {
        src.iter().position(|&b| b == b'\r')
    }
}

/// Identifies an offset of a multi-byte character in a `SourceFile`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct MultiByteChar {
    /// The absolute offset of the character in the `SourceMap`.
    pub pos: BytePos,
    /// The number of bytes, `>= 2`.
    pub bytes: u8,
}

/// Finds all newlines, multi-byte characters, and non-narrow characters in a
/// SourceFile.
///
/// This function will use an SSE2 enhanced implementation if hardware support
/// is detected at runtime.
fn analyze_source_file(src: &str, source_file_start_pos: BytePos) -> (Vec<BytePos>, Vec<MultiByteChar>) {
    let mut lines = vec![source_file_start_pos];
    let mut multi_byte_chars = vec![];

    let mut i = 0;
    let src_bytes = src.as_bytes();

    while i < src.len() {
        let byte = src_bytes[i];

        // How much to advance to get to the next UTF-8 char in the string.
        let mut char_len = 1;

        let pos = BytePos::from_usize(i) + source_file_start_pos;

        if let b'\n' = byte {
            lines.push(pos + BytePos(1));
        } else if byte >= 127 {
            // The slow path:
            // This is either ASCII control character "DEL" or the beginning of
            // a multibyte char. Just decode to `char`.
            let c = (src[i..]).chars().next().unwrap();
            char_len = c.len_utf8();

            if char_len > 1 {
                assert!((2..=4).contains(&char_len));
                let bytes = char_len as u8;
                let mbc = MultiByteChar { pos, bytes };
                multi_byte_chars.push(mbc);
            }
        }

        i += char_len;
    }

    // The code above optimistically registers a new line *after* each \n it encounters.
    // If that point is already outside the source_file, remove it again.
    if let Some(&last_line_start) = lines.last() {
        let source_file_end = source_file_start_pos + BytePos::from_usize(src.len());
        assert!(source_file_end >= last_line_start);
        if last_line_start == source_file_end {
            lines.pop();
        }
    }

    (lines, multi_byte_chars)
}
