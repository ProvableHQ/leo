// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use colored::*;
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

mod cursor;
use cursor::*;

mod interpreter;
use interpreter::*;

mod cursor_aleo;

mod value;
use value::*;

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

You may also use one letter abbreviations for these commands, such as #i.

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

Input history is available - use the up and down arrow keys.
";

fn parse_breakpoint(s: &str) -> Option<Breakpoint> {
    let strings: Vec<&str> = s.split_whitespace().collect();
    if strings.len() == 2 {
        let mut program = strings[0].to_string();
        if program.ends_with(".aleo") {
            program.truncate(program.len() - 5);
        }
        if let Ok(line) = strings[1].parse::<usize>() {
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
) -> Result<()> {
    let mut interpreter = Interpreter::new(leo_filenames.iter(), aleo_filenames.iter(), signer, block_height)?;
    println!("{}", INTRO);

    let mut history = dialoguer::BasicHistory::new();
    loop {
        if let Some(v) = interpreter.view_current_in_context() {
            println!("{}:\n{v}", "Prepared to evaluate".bold());
        } else if let Some(v) = interpreter.view_current() {
            println!("{}:\n{v}", "Prepared to evaluate".bold());
        }

        for (i, future) in interpreter.cursor.futures.iter().enumerate() {
            println!("{i:>4}: {future}");
        }

        interpreter.update_watchpoints()?;

        for (i, watchpoint) in interpreter.watchpoints.iter().enumerate() {
            println!(
                "{i:>4}: {:>20} = {}",
                watchpoint.code,
                watchpoint.last_result.as_ref().map(|s| s.as_str()).unwrap_or("")
            );
        }

        let user_input: String = dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Command?")
            .history_with(&mut history)
            .interact_text()
            .unwrap();

        let (command, rest) = tokenize_user_input(&user_input);

        let action = match (command, rest) {
            ("", "") => continue,
            ("#h" | "#help", "") => {
                println!("{}", HELP);
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
                        println!("No such future");
                        continue;
                    }
                    InterpreterAction::RunFuture(num)
                } else {
                    println!("Failed to parse future");
                    continue;
                }
            }
            ("#b" | "#break", rest) => {
                let Some(breakpoint) = parse_breakpoint(rest) else {
                    println!("Failed to parse breakpoint");
                    continue;
                };
                InterpreterAction::Breakpoint(breakpoint)
            }
            ("#p" | "#print", rest) => {
                let without_r = rest.strip_prefix("r").unwrap_or(rest);
                if let Ok(num) = without_r.parse::<u64>() {
                    InterpreterAction::PrintRegister(num)
                } else {
                    println!("Failed to parse register number {rest}");
                    continue;
                }
            }
            ("#w" | "#watch", rest) => InterpreterAction::Watch(rest.to_string()),
            ("", rest) => InterpreterAction::LeoInterpretOver(rest.to_string()),
            _ => {
                println!("Failed to parse command {user_input}");
                continue;
            }
        };

        match interpreter.action(action) {
            Ok(Some(value)) => {
                println!("{}: {}\n", "Result".bold(), format!("{value}").bright_cyan());
            }
            Ok(None) => {}
            Err(LeoError::InterpreterHalt(interpreter_halt)) => println!("Halted: {interpreter_halt}"),
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
