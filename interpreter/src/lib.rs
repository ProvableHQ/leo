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

use leo_ast::{Ast, Node as _, NodeBuilder};
use leo_errors::{InterpreterHalt, LeoError, Result};
use leo_span::{Span, source_map::FileName, symbol::with_session_globals};

use snarkvm::prelude::{Program, TestnetV0};

use std::{
    collections::HashMap,
    fmt::{Display, Write as _},
    fs,
    path::{Path, PathBuf},
};

#[cfg(test)]
mod test;

mod util;
use util::*;

mod core_functions;
pub use core_functions::{CoreFunctionHelper, evaluate_core_function};

mod cursor;
use cursor::*;
pub use cursor::{evaluate_binary, evaluate_unary, literal_to_value};

mod interpreter;
use interpreter::*;

mod cursor_aleo;

mod value;
use value::*;
pub use value::{StructContents, Value};

mod ui;
use ui::Ui;

mod dialoguer_input;

mod ratatui_ui;

const INTRO: &str = "This is the Leo Interpreter. Try the command `#help`.";

const HELP: &str = "
You probably want to start by running a function or transition.
For instance
#into program.aleo/main()
Once a function is running, commands include
#into    to evaluate into the next expression or statement;
#step    to take one step towards evaluating the current expression or statement;
#over    to complete evaluating the current expression or statement;
#run     to finish evaluating
#quit    to quit the interpreter.

You can set a breakpoint with
#break program_name line_number

When executing Aleo VM code, you can print the value of a register like this:
#print 2

Some of the commands may be run with one letter abbreviations, such as #i.

Note that this interpreter is not line oriented as in many common debuggers;
rather it is oriented around expressions and statements.
As you step into code, individual expressions or statements will
be evaluated one by one, including arguments of function calls.

You may simply enter Leo expressions or statements on the command line
to evaluate. For instance, if you want to see the value of a variable w:
w
If you want to set w to a new value:
w = z + 2u8;

Note that statements (like the assignment above) must end with a semicolon.

If there are futures available to be executed, they will be listed by
numerical index, and you may run them using `#future` (or `#f`); for instance
#future 0

The interpreter begins in a global context, not in any Leo program. You can set
the current program with

#set_program program_name

This allows you to refer to structs and other items in the indicated program.

The interpreter may enter an invalid state, often due to Leo code entered at the
REPL. In this case, you may use the command

#restore

Which will restore to the last saved state of the interpreter. Any time you
enter Leo code at the prompt, interpreter state is saved.

Input history is available - use the up and down arrow keys.
";

fn parse_breakpoint(s: &str) -> Option<Breakpoint> {
    let strings: Vec<&str> = s.split_whitespace().collect();
    if strings.len() == 2 {
        if let Ok(line) = strings[1].parse::<usize>() {
            let program = strings[0].strip_suffix(".aleo").unwrap_or(strings[0]).to_string();
            return Some(Breakpoint { program, line });
        }
    }
    None
}

/// Load all the Leo source files indicated and open the interpreter
/// to commands from the user.
pub fn interpret(
    leo_filenames: &[PathBuf],
    aleo_filenames: &[PathBuf],
    signer: SvmAddress,
    block_height: u32,
    tui: bool,
) -> Result<()> {
    let mut interpreter = Interpreter::new(leo_filenames.iter(), aleo_filenames.iter(), signer, block_height)?;

    let mut user_interface: Box<dyn Ui> =
        if tui { Box::new(ratatui_ui::RatatuiUi::new()) } else { Box::new(dialoguer_input::DialoguerUi::new()) };

    let mut code = String::new();
    let mut futures = Vec::new();
    let mut watchpoints = Vec::new();
    let mut message = INTRO.to_string();
    let mut result = String::new();

    loop {
        code.clear();
        futures.clear();
        watchpoints.clear();

        let (code, highlight) = if let Some((code, lo, hi)) = interpreter.view_current_in_context() {
            (code.to_string(), Some((lo, hi)))
        } else if let Some(v) = interpreter.view_current() {
            (v.to_string(), None)
        } else {
            ("".to_string(), None)
        };

        futures.extend(interpreter.cursor.futures.iter().map(|f| f.to_string()));

        interpreter.update_watchpoints()?;

        watchpoints.extend(interpreter.watchpoints.iter().map(|watchpoint| {
            format!("{:>15} = {}", watchpoint.code, if let Some(s) = &watchpoint.last_result { &**s } else { "?" })
        }));

        let user_data = ui::UserData {
            code: &code,
            highlight,
            message: &message,
            futures: &futures,
            watchpoints: &watchpoints,
            result: if result.is_empty() { None } else { Some(&result) },
        };

        user_interface.display_user_data(&user_data);

        message.clear();
        result.clear();

        let user_input = user_interface.receive_user_input();

        let (command, rest) = tokenize_user_input(&user_input);

        let action = match (command, rest) {
            ("", "") => continue,
            ("#h" | "#help", "") => {
                message = HELP.to_string();
                continue;
            }
            ("#i" | "#into", "") => InterpreterAction::Into,
            ("#i" | "#into", rest) => InterpreterAction::LeoInterpretInto(rest.into()),
            ("#s" | "#step", "") => InterpreterAction::Step,
            ("#o" | "#over", "") => InterpreterAction::Over,
            ("#r" | "#run", "") => InterpreterAction::Run,
            ("#q" | "#quit", "") => return Ok(()),
            ("#f" | "#future", rest) => {
                if let Ok(num) = rest.trim().parse::<usize>() {
                    if num >= interpreter.cursor.futures.len() {
                        message = "No such future.".to_string();
                        continue;
                    }
                    InterpreterAction::RunFuture(num)
                } else {
                    message = "Failed to parse future.".to_string();
                    continue;
                }
            }
            ("#restore", "") => {
                if !interpreter.restore_cursor() {
                    message = "No saved state to restore".to_string();
                }
                continue;
            }
            ("#b" | "#break", rest) => {
                let Some(breakpoint) = parse_breakpoint(rest) else {
                    message = "Failed to parse breakpoint".to_string();
                    continue;
                };
                InterpreterAction::Breakpoint(breakpoint)
            }
            ("#p" | "#print", rest) => {
                let without_r = rest.strip_prefix("r").unwrap_or(rest);
                if let Ok(num) = without_r.parse::<u64>() {
                    InterpreterAction::PrintRegister(num)
                } else {
                    message = "Failed to parse register number".to_string();
                    continue;
                }
            }
            ("#w" | "#watch", rest) => InterpreterAction::Watch(rest.to_string()),
            ("#set_program", rest) => {
                interpreter.cursor.set_program(rest);
                continue;
            }
            ("", rest) => InterpreterAction::LeoInterpretOver(rest.to_string()),
            _ => {
                message = "Failed to parse command".to_string();
                continue;
            }
        };

        if matches!(action, InterpreterAction::LeoInterpretInto(..) | InterpreterAction::LeoInterpretOver(..)) {
            interpreter.save_cursor();
        }

        match interpreter.action(action) {
            Ok(Some(value)) => {
                result = value.to_string();
            }
            Ok(None) => {}
            Err(LeoError::InterpreterHalt(interpreter_halt)) => {
                message = format!("Halted: {interpreter_halt}");
            }
            Err(e) => return Err(e),
        }
    }
}

fn tokenize_user_input(input: &str) -> (&str, &str) {
    let input = input.trim();

    if !input.starts_with("#") {
        return ("", input);
    }

    let Some((first, rest)) = input.split_once(' ') else {
        return (input, "");
    };

    (first.trim(), rest.trim())
}
