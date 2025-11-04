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

use biome_formatter::{
    Buffer,
    Format,
    FormatElement,
    SimpleFormatContext,
    prelude::{
        Tag,
        dynamic_text,
        empty_line,
        hard_line_break,
        if_group_breaks,
        soft_line_break,
        soft_line_break_or_space,
        space,
        tag::{Group, GroupMode},
        text,
    },
    write,
};
use leo_parser_lossless::SyntaxNode;

use crate::{Formatter, Output};

impl Formatter<'_, '_> {
    pub(crate) fn node_with_trivia(&mut self, node: &SyntaxNode<'_>, line_breaks: u32) -> Output {
        self.push_snippet(node.text)?;
        self.consolidate_trivia(&node.children[..], line_breaks)?;
        Ok(())
    }

    pub(crate) fn format_collection(
        &mut self,
        nodes: &[SyntaxNode<'_>],
        always_expand: bool,
        spaces: bool,
        format_func: impl Fn(&mut Self, &SyntaxNode<'_>) -> Output,
    ) -> Output {
        let left_delimiter = nodes.first().unwrap();
        self.push_snippet(left_delimiter.text)?;

        let block = if always_expand && nodes.len() > 2 {
            Self::scope
        } else if spaces {
            Self::soft_scope_or_spaces
        } else {
            Self::soft_scope
        };

        if nodes.len() > 2 {
            block(self, |slf: &mut Self| {
                let [_, nodes @ .., _] = nodes else { panic!("Can't happen") };
                slf.consolidate_trivia(&left_delimiter.children, 0)?;
                if let Some(first) = nodes.first() {
                    format_func(slf, first)?;
                }

                if let Some(rest) = nodes.get(1..) {
                    let mut chunks = rest.chunks_exact(2);
                    for comma_with_node in &mut chunks {
                        slf.node_with_trivia(&comma_with_node[0], 0)?;
                        if !slf.last_line() {
                            slf.push(&soft_line_break_or_space())?;
                        } else {
                            slf.space()?;
                        }
                        format_func(slf, &comma_with_node[1])?;
                    }

                    slf.push(&if_group_breaks(&text(",")))?;
                    // This can only be a separator
                    if let Some(rem) = chunks.remainder().first() {
                        slf.consolidate_trivia(&rem.children, 1)?;
                    }
                } else {
                    slf.push(&if_group_breaks(&text(",")))?;
                }

                Ok(())
            })?;
        } else {
            self.consolidate_trivia(&left_delimiter.children, 0)?;
        }

        let right_delimiter = nodes.last().unwrap();
        self.node_with_trivia(right_delimiter, 1)
    }

    pub(crate) fn group(&mut self, func: impl FnOnce(&mut Self) -> Output) -> Output {
        self.formatter.write_element(FormatElement::Tag(Tag::StartGroup(
            Group::new().with_id(None).with_mode(GroupMode::Flat),
        )))?;

        func(self)?;

        self.formatter.write_element(FormatElement::Tag(Tag::EndGroup))
    }

    pub(crate) fn scope(&mut self, func: impl FnOnce(&mut Self) -> Output) -> Output {
        self.block(IndentMode::Block, func)
    }

    pub(crate) fn soft_scope(&mut self, func: impl FnOnce(&mut Self) -> Output) -> Output {
        self.group(|slf| slf.block(IndentMode::Soft, func))
    }

    pub(crate) fn soft_scope_or_spaces(&mut self, func: impl FnOnce(&mut Self) -> Output) -> Output {
        self.group(|slf| slf.block(IndentMode::SoftSpace, func))
    }

    pub(crate) fn soft_indent_or_space(&mut self, func: impl FnOnce(&mut Self) -> Output) -> Output {
        self.group(|slf| slf.block(IndentMode::SoftLineOrSpace, func))
    }

    pub(crate) fn block(&mut self, mode: IndentMode, func: impl FnOnce(&mut Self) -> Output) -> Output {
        let snapshot = self.formatter.snapshot();

        self.formatter.write_element(FormatElement::Tag(Tag::StartIndent))?;

        match mode {
            IndentMode::Soft => write!(self.formatter, [soft_line_break()])?,
            IndentMode::Block => write!(self.formatter, [hard_line_break()])?,
            IndentMode::SoftLine => write!(self.formatter, [soft_line_break()])?,
            IndentMode::SoftLineOrSpace | IndentMode::SoftSpace => {
                write!(self.formatter, [soft_line_break_or_space()])?
            }
        }

        let is_empty = {
            let len = self.formatter.elements().len();
            func(self)?;
            let new_len = self.formatter.elements().len();
            new_len == len
        };

        if is_empty {
            self.formatter.restore_snapshot(snapshot);
            return Ok(());
        }

        self.formatter.write_element(FormatElement::Tag(Tag::EndIndent))?;

        match mode {
            IndentMode::Soft => write!(self.formatter, [soft_line_break()]),
            IndentMode::Block => write!(self.formatter, [hard_line_break()]),
            IndentMode::SoftLine => Ok(()),
            IndentMode::SoftSpace => write!(self.formatter, [soft_line_break_or_space()]),
            IndentMode::SoftLineOrSpace => Ok(()),
        }
    }

    pub(crate) fn push(&mut self, node: &(impl Format<SimpleFormatContext> + ?Sized)) -> Output {
        node.fmt(self.formatter)
    }

    pub(crate) fn space(&mut self) -> Output {
        space().fmt(self.formatter)
    }

    pub(crate) fn hard_line(&mut self) -> Output {
        hard_line_break().fmt(self.formatter)?;
        self.bump_lines();
        Ok(())
    }

    pub(crate) fn push_snippet(&mut self, snippet: &str) -> Output {
        self.reset_lines();
        self.push(&dynamic_text(snippet, Default::default()))
    }

    pub(crate) fn last_line(&self) -> bool {
        self.last_lines == 1
    }

    pub(crate) fn _last_double_line(&self) -> bool {
        self.last_lines == 2
    }

    pub(crate) fn reset_lines(&mut self) {
        self.last_lines = 0;
    }

    pub(crate) fn bump_lines(&mut self) {
        self.last_lines += 1;
    }

    pub(crate) fn maybe_bump_line_else_ignore(&mut self) -> Output {
        match self.last_lines {
            0 => self.hard_line()?,

            1 | 2 => {}

            _ => panic!("not allowed"),
        }

        self.reset_lines();

        Ok(())
    }

    pub(crate) fn maybe_bump_line(&mut self) -> Output {
        match self.last_lines {
            0 => self.hard_line()?,

            1 => {}

            _ => panic!("not allowed"),
        }

        self.reset_lines();

        Ok(())
    }

    pub(crate) fn empty_line(&mut self) -> Output {
        empty_line().fmt(self.formatter)
    }

    pub(crate) fn maybe_bump_lines(&mut self) -> Output {
        match self.last_lines {
            0 | 1 => {
                self.empty_line()?;
            }

            2 => {}

            _ => panic!("not allowed"),
        }

        self.reset_lines();

        Ok(())
    }

    pub(crate) fn _line_suffix(&mut self, func: impl Fn(&mut Self) -> Output) -> Output {
        self.formatter.write_element(FormatElement::Tag(Tag::StartLineSuffix))?;
        func(self)?;
        self.formatter.write_element(FormatElement::Tag(Tag::EndLineSuffix))?;
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum IndentMode {
    Soft,
    Block,
    SoftLine,
    SoftSpace,
    SoftLineOrSpace,
}
