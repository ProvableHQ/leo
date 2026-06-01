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

#[cfg(not(target_arch = "wasm32"))]
use leo_lang::cli::*;
#[cfg(not(target_arch = "wasm32"))]
use leo_span::create_session_if_not_set_then;

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;

#[cfg(not(target_arch = "wasm32"))]
fn set_panic_hook() {
    std::panic::set_hook({
        Box::new(move |e| {
            eprintln!("thread `{}` {}", std::thread::current().name().unwrap_or("<unnamed>"), e);
            eprintln!("stack backtrace: \n{:?}", backtrace::Backtrace::new());
            eprintln!("error: internal compiler error: unexpected panic\n");
            eprintln!("note: the compiler unexpectedly panicked. this is a bug.\n");
            eprintln!(
                "note: we would appreciate a bug report: https://github.com/ProvableHQ/leo/issues/new?labels=bug,panic&template=bug.md&title=[Bug]\n"
            );
            eprintln!(
                "note: leo {} running on {} {}\n",
                env!("LEO_VERSION_STRING"),
                sys_info::os_type().unwrap_or_else(|e| e.to_string()),
                sys_info::os_release().unwrap_or_else(|e| e.to_string()),
            );
            eprintln!("note: compiler args: {}\n", std::env::args().collect::<Vec<_>>().join(" "));
            eprintln!("note: compiler flags: {:?}\n", CLI::parse());
        })
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    set_panic_hook();
    create_session_if_not_set_then(|_| handle_error(run_with_args(CLI::parse())));
}

// The `leo` binary is native-only — all command implementations rely on
// snarkVM's umbrella, HTTP clients, the real filesystem, and so on. The
// `leo-lang` *library* (containing the wasm-buildable `options` module) is
// what `leo-wasm` consumes from `wasm32-unknown-unknown`.
#[cfg(target_arch = "wasm32")]
fn main() {}
