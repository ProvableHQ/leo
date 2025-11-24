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

use std::{ffi::OsStr, fs, path::Path};

use anyhow::Result;
use biome_formatter::{
    IndentStyle,
    IndentWidth,
    LineWidth,
    SimpleFormatContext,
    SimpleFormatOptions,
    format,
    prelude::format_once,
};
use colored::Colorize;
use leo_errors::Handler;
use leo_parser_lossless::{parse_main, parse_module};
use similar::{ChangeTag, TextDiff};
use walkdir::WalkDir;

const DEFAULT_LINE_WIDTH: u16 = 80;
const UNIFIED_DIFF_HUNK_RADIUS: usize = 3;

pub struct Formatter<'a, 'b> {
    pub(crate) last_lines: u32,
    pub(crate) formatter: &'a mut biome_formatter::formatter::Formatter<'b, SimpleFormatContext>,
}

impl Formatter<'_, '_> {
    pub fn format_context(line_width: u16, indent_width: u8, should_indent_style_tabs: bool) -> SimpleFormatContext {
        SimpleFormatContext::new(SimpleFormatOptions {
            line_width: LineWidth::try_from(line_width).unwrap(),
            indent_width: IndentWidth::try_from(indent_width).unwrap(),
            indent_style: if should_indent_style_tabs { IndentStyle::Tab } else { IndentStyle::Space },
            ..SimpleFormatOptions::default()
        })
    }

    pub fn default_format_context() -> SimpleFormatContext {
        SimpleFormatContext::new(SimpleFormatOptions {
            line_width: LineWidth::try_from(DEFAULT_LINE_WIDTH).unwrap(),
            indent_width: IndentWidth::try_from(4).unwrap(),
            indent_style: IndentStyle::Space,
            ..SimpleFormatOptions::default()
        })
    }

    pub fn format_directory(
        entry_file_path: impl AsRef<Path>,
        modules_directory: Option<impl AsRef<Path>>,
        context_provider: impl Fn() -> SimpleFormatContext,
        check_diff: bool,
    ) -> Result<()> {
        // Read the contents of the main source file.
        let source = if entry_file_path.as_ref().exists() { Some(fs::read_to_string(&entry_file_path)?) } else { None };

        // Walk all files under source_directory recursively, excluding the main source file itself.
        let files = if let Some(dir) = modules_directory {
            WalkDir::new(dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path() != entry_file_path.as_ref()
                        && e.path().extension() == Some(OsStr::new("leo"))
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let mut module_sources = Vec::new(); // Keep Strings alive for valid borrowing
        //let mut modules = Vec::new(); // Parsed (source, filename) tuples for compilation
        let mut module_file_paths = Vec::new();

        // Read all module files and store their contents
        for file in &files {
            let path = file.path();
            let source = fs::read_to_string(path)?;
            module_sources.push(source); // Keep the String alive
            module_file_paths.push(path);
        }

        let (formatted_main, formatted_modules) = Self::format(&source, &module_sources, context_provider)?;

        assert_eq!(module_sources.len(), formatted_modules.len(), "This should not happpen.");

        let (main_diff, module_diffs) = Self::diff(&source, &module_sources, &formatted_main, &formatted_modules);

        if check_diff {
            if entry_file_path.as_ref().exists() {
                Self::print_diff(&main_diff.unwrap(), entry_file_path.as_ref());
            }

            for (diff, path) in module_diffs.iter().zip(module_file_paths) {
                Self::print_diff(diff, path);
            }
        } else {
            // Todo: maybe writing to disk can be optimized by the incremental thingy.
            if entry_file_path.as_ref().exists()
                && main_diff.unwrap().iter_all_changes().any(|c| !matches!(c.tag(), ChangeTag::Equal))
            {
                fs::write(entry_file_path, formatted_main.as_ref().unwrap())?;
            }

            for ((formatted_module, path), diff) in formatted_modules.iter().zip(module_file_paths).zip(module_diffs) {
                if diff.iter_all_changes().any(|c| !matches!(c.tag(), ChangeTag::Equal)) {
                    fs::write(path, formatted_module)?;
                }
            }
        }

        Ok(())
    }

    fn format(
        main: &Option<String>,
        modules: &Vec<String>,
        context_provider: impl Fn() -> SimpleFormatContext,
    ) -> Result<(Option<String>, Vec<String>)> {
        let formatted_main = if let Some(m) = main {
            let node = parse_main(Handler::default(), m, 0)?;
            let fm = format!(context_provider(), [format_once(|f| {
                Formatter { last_lines: 0, formatter: f }.format_main(&node)
            })])?;

            Some(fm.print()?.into_code())
        } else {
            None
        };

        let mut formatted_modules = Vec::new();
        for module in modules {
            let node = parse_module(Handler::default(), module, 0)?;
            let fm = format!(context_provider(), [format_once(|f| {
                Formatter { last_lines: 0, formatter: f }.format_module(&node)
            })])?;

            formatted_modules.push(fm.print()?.into_code());
        }

        Ok((formatted_main, formatted_modules))
    }

    fn diff<'a>(
        main_source: &'a Option<String>,
        module_sources: &'a [String],
        formatted_main: &'a Option<String>,
        formatted_modules: &'a Vec<String>,
    ) -> (Option<TextDiff<'a, 'a, 'a, str>>, Vec<TextDiff<'a, 'a, 'a, str>>) {
        let main_diff = match (main_source, formatted_main) {
            (Some(s), Some(f)) => Some(TextDiff::from_lines(s.as_str(), f.as_str())),
            (None, None) => None,
            _ => panic!("This should not happpen."),
        };

        assert_eq!(module_sources.len(), formatted_modules.len(), "This should not happpen.");

        let module_diffs = module_sources
            .iter()
            .zip(formatted_modules)
            .map(|(s, f)| TextDiff::from_lines(s.as_str(), f.as_str()))
            .collect();

        (main_diff, module_diffs)
    }

    fn print_diff<'a>(diff: &TextDiff<'a, 'a, 'a, str>, path: &Path) {
        for hunk in diff.unified_diff().context_radius(UNIFIED_DIFF_HUNK_RADIUS).iter_hunks() {
            let mut hunk_to_print = String::new();
            let mut diff_line = None;
            for change in hunk.iter_changes() {
                match change.tag() {
                    ChangeTag::Delete => hunk_to_print.push_str(&std::format!("{}{}", change.tag(), change).red()),
                    ChangeTag::Equal => hunk_to_print.push_str(std::format!("{}{}", change.tag(), change).as_str()),
                    ChangeTag::Insert => hunk_to_print.push_str(&std::format!("{}{}", change.tag(), change).green()),
                }
                if diff_line.is_none() && change.tag() != ChangeTag::Equal {
                    diff_line = change.old_index();
                }
            }

            println!("Diff in {:?}:{}", path, diff_line.map(|i| ToString::to_string(&i)).unwrap_or("".to_string()));
            println!("{hunk_to_print}");
        }
    }
}
