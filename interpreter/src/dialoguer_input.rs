// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use colored::*;

pub struct DialoguerUi {
    history: dialoguer::BasicHistory,
}

impl DialoguerUi {
    pub fn new() -> Self {
        DialoguerUi { history: dialoguer::BasicHistory::new() }
    }
}

impl Ui for DialoguerUi {
    fn display_user_data(&mut self, data: &UserData<'_>) {
        if let Some(result) = data.result {
            println!("{}: {}", "Result".bold(), result.bright_cyan());
        }
        println!("{}", data.message);
        if let Some(highlight_span) = data.highlight {
            let first = data.code.get(0..highlight_span.0).expect("spans should be valid");
            let second = data.code.get(highlight_span.0..highlight_span.1).expect("spans should be valid");
            let third = data.code.get(highlight_span.1..).expect("spans should be valid");
            println!("{first}{}{third}", second.red());
        } else {
            println!("{}", data.code);
        }

        for (i, future) in data.futures.iter().enumerate() {
            println!("{i:>4}: {future}");
        }

        for (i, watchpoint) in data.watchpoints.iter().enumerate() {
            println!("{i:>4}: {watchpoint}");
        }
    }

    fn receive_user_input(&mut self) -> String {
        dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Command?")
            .history_with(&mut self.history)
            .interact_text()
            .unwrap()
    }
}
