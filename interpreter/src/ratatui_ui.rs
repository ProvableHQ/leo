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

use super::ui::{Ui, UserData};

use std::{cmp, collections::VecDeque, io::Stdout, mem};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    Frame,
    Terminal,
    prelude::{
        Buffer,
        Constraint,
        CrosstermBackend,
        Direction,
        Layout,
        Line,
        Modifier,
        Rect,
        Span,
        Style,
        Stylize as _,
    },
    text::Text,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Default)]
struct DrawData {
    code: String,
    highlight: Option<(usize, usize)>,
    result: String,
    watchpoints: Vec<String>,
    message: String,
    prompt: Prompt,
}

pub struct RatatuiUi {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    data: DrawData,
}

impl Drop for RatatuiUi {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

impl RatatuiUi {
    pub fn new() -> Self {
        RatatuiUi { terminal: ratatui::init(), data: Default::default() }
    }
}

fn append_lines<'a>(
    lines: &mut Vec<Line<'a>>,
    mut last_chunk: Option<Line<'a>>,
    string: &'a str,
    style: Style,
) -> Option<Line<'a>> {
    let mut line_iter = string.lines().peekable();
    while let Some(line) = line_iter.next() {
        let this_span = Span::styled(line, style);
        let mut real_last_chunk = mem::take(&mut last_chunk).unwrap_or_else(|| Line::raw(""));
        real_last_chunk.push_span(this_span);
        if line_iter.peek().is_some() {
            lines.push(real_last_chunk);
        } else if string.ends_with('\n') {
            lines.push(real_last_chunk);
            return None;
        } else {
            return Some(real_last_chunk);
        }
    }

    last_chunk
}

fn code_text(s: &str, highlight: Option<(usize, usize)>) -> (Text, usize) {
    let Some((lo, hi)) = highlight else {
        return (Text::from(s), 0);
    };

    let s1 = s.get(..lo).expect("should be able to split text");
    let s2 = s.get(lo..hi).expect("should be able to split text");
    let s3 = s.get(hi..).expect("should be able to split text");

    let mut lines = Vec::new();

    let s1_chunk = append_lines(&mut lines, None, s1, Style::default());
    let line = lines.len();
    let s2_chunk = append_lines(&mut lines, s1_chunk, s2, Style::new().red());
    let s3_chunk = append_lines(&mut lines, s2_chunk, s3, Style::default());

    if let Some(chunk) = s3_chunk {
        lines.push(chunk);
    }

    (Text::from(lines), line)
}

struct DebuggerLayout {
    code: Rect,
    result: Rect,
    watchpoints: Rect,
    user_input: Rect,
    message: Rect,
}

impl DebuggerLayout {
    fn new(total: Rect) -> Self {
        let overall_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),   // Code
                Constraint::Length(6), // Result and watchpoints
                Constraint::Length(3), // Message
                Constraint::Length(3), // User input
            ])
            .split(total);
        let code = overall_layout[0];
        let middle = overall_layout[1];
        let message = overall_layout[2];
        let user_input = overall_layout[3];

        let middle = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(middle);

        DebuggerLayout { code, result: middle[0], watchpoints: middle[1], user_input, message }
    }
}

#[derive(Debug, Default)]
struct Prompt {
    history: VecDeque<String>,
    history_index: usize,
    current: String,
    cursor: usize,
}

impl<'a> Widget for &'a Prompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut plain = || {
            Text::raw(&self.current).render(area, buf);
        };

        if self.cursor >= self.current.len() {
            let span1 = Span::raw(&self.current);
            let span2 = Span::styled(" ", Style::new().add_modifier(Modifier::REVERSED));
            Text::from(Line::from_iter([span1, span2])).render(area, buf);
            return;
        }

        let Some(pre) = self.current.get(..self.cursor) else {
            plain();
            return;
        };

        let Some(c) = self.current.get(self.cursor..self.cursor + 1) else {
            plain();
            return;
        };

        let Some(post) = self.current.get(self.cursor + 1..) else {
            plain();
            return;
        };

        Text::from(Line::from_iter([
            Span::raw(pre),
            Span::styled(c, Style::new().add_modifier(Modifier::REVERSED)),
            Span::raw(post),
        ]))
        .render(area, buf);
    }
}

impl Prompt {
    fn handle_key(&mut self, key: KeyCode, control: bool) -> Option<String> {
        match (key, control) {
            (KeyCode::Enter, _) => {
                self.history.push_back(mem::take(&mut self.current));
                self.history_index = self.history.len();
                return self.history.back().cloned();
            }
            (KeyCode::Backspace, _) => self.backspace(),
            (KeyCode::Left, _) => self.left(),
            (KeyCode::Right, _) => self.right(),
            (KeyCode::Up, _) => self.history_prev(),
            (KeyCode::Down, _) => self.history_next(),
            (KeyCode::Delete, _) => self.delete(),
            (KeyCode::Char(c), false) => self.new_character(c),
            (KeyCode::Char('a'), true) => self.beginning_of_line(),
            (KeyCode::Char('e'), true) => self.end_of_line(),
            _ => {}
        }

        None
    }

    fn new_character(&mut self, c: char) {
        if self.cursor >= self.current.len() {
            self.current.push(c);
            self.cursor = self.current.len();
        } else {
            let Some(pre) = self.current.get(..self.cursor) else {
                return;
            };
            let Some(post) = self.current.get(self.cursor..) else {
                return;
            };
            let mut with_char = format!("{pre}{c}");
            self.cursor = with_char.len();
            with_char.push_str(post);
            self.current = with_char;
        }
        self.check_history();
    }

    fn right(&mut self) {
        self.cursor = cmp::min(self.cursor + 1, self.current.len());
    }

    fn left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }

        if self.cursor >= self.current.len() {
            self.current.pop();
            self.cursor = self.current.len();
            return;
        }

        let Some(pre) = self.current.get(..self.cursor - 1) else {
            return;
        };
        let Some(post) = self.current.get(self.cursor..) else {
            return;
        };
        self.cursor -= 1;

        let s = format!("{pre}{post}");

        self.current = s;

        self.check_history();
    }

    fn delete(&mut self) {
        if self.cursor + 1 >= self.current.len() {
            return;
        }

        let Some(pre) = self.current.get(..self.cursor) else {
            return;
        };
        let Some(post) = self.current.get(self.cursor + 1..) else {
            return;
        };

        let s = format!("{pre}{post}");

        self.current = s;

        self.check_history();
    }

    fn beginning_of_line(&mut self) {
        self.cursor = 0;
    }

    fn end_of_line(&mut self) {
        self.cursor = self.current.len();
    }

    fn history_next(&mut self) {
        self.history_index += 1;
        if self.history_index > self.history.len() {
            self.history_index = 0;
        }
        self.current = self.history.get(self.history_index).cloned().unwrap_or(String::new());
    }

    fn history_prev(&mut self) {
        if self.history_index == 0 {
            self.history_index = self.history.len();
        } else {
            self.history_index -= 1;
        }
        self.current = self.history.get(self.history_index).cloned().unwrap_or(String::new());
    }

    fn check_history(&mut self) {
        const MAX_HISTORY: usize = 50;

        while self.history.len() > MAX_HISTORY {
            self.history.pop_front();
        }

        self.history_index = self.history.len();
    }
}

fn render_titled<W: Widget>(frame: &mut Frame, widget: W, title: &str, area: Rect) {
    let block = Block::bordered().title(title);
    frame.render_widget(widget, block.inner(area));
    frame.render_widget(block, area);
}

impl DrawData {
    fn draw(&mut self, frame: &mut Frame) {
        let layout = DebuggerLayout::new(frame.area());

        let (code, line) = code_text(&self.code, self.highlight);
        let p = Paragraph::new(code).scroll((line.saturating_sub(4) as u16, 0));
        render_titled(frame, p, "code", layout.code);

        render_titled(frame, Text::raw(&self.result), "Result", layout.result);

        render_titled(frame, Text::from_iter(self.watchpoints.iter().map(|s| &**s)), "Watchpoints", layout.watchpoints);

        render_titled(frame, Text::raw(&self.message), "Message", layout.message);

        render_titled(frame, &self.prompt, "Command:", layout.user_input);
    }
}

impl Ui for RatatuiUi {
    fn display_user_data(&mut self, data: &UserData<'_>) {
        self.data.code = data.code.to_string();
        self.data.highlight = data.highlight;
        self.data.result = data.result.map(|s| s.to_string()).unwrap_or_default();
        self.data.watchpoints.clear();
        self.data.watchpoints.extend(data.watchpoints.iter().enumerate().map(|(i, s)| format!("{i:>2} {s}")));
        self.data.message = data.message.to_string();
    }

    fn receive_user_input(&mut self) -> String {
        loop {
            self.terminal.draw(|frame| self.data.draw(frame)).expect("failed to draw frame");
            if let Event::Key(key_event) = event::read().expect("event") {
                let control = key_event.modifiers.contains(KeyModifiers::CONTROL);
                if let Some(string) = self.data.prompt.handle_key(key_event.code, control) {
                    return string;
                }
            }
        }
    }
}
