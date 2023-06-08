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

use leo_lang::cli::*;
use leo_span::symbol::create_session_if_not_set_then;

use clap::Parser;

fn set_panic_hook() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook({
        Box::new(move |e| {
            eprintln!("thread `{}` {}", std::thread::current().name().unwrap_or("<unnamed>"), e);
            eprintln!("stack backtrace: \n{:?}", backtrace::Backtrace::new());
            eprintln!("error: internal compiler error: unexpected panic\n");
            eprintln!("note: the compiler unexpectedly panicked. this is a bug.\n");
            eprintln!(
                "note: we would appreciate a bug report: https://github.com/AleoHQ/leo/issues/new?labels=bug,panic&template=bug.md&title=[Bug]\n"
            );
            eprintln!(
                "note: {} {} running on {} {}\n",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                sys_info::os_type().unwrap_or_else(|e| e.to_string()),
                sys_info::os_release().unwrap_or_else(|e| e.to_string()),
            );
            eprintln!("note: compiler args: {}\n", std::env::args().collect::<Vec<_>>().join(" "));
            eprintln!("note: compiler flags: {:?}\n", CLI::parse());
        })
    });
}

fn main() {
    set_panic_hook();
    create_session_if_not_set_then(|_| handle_error(run_with_args(CLI::parse())));
}
