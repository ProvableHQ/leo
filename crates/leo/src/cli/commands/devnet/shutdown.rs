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

use crossbeam_channel::Sender;
use std::{io, thread};

pub fn install_shutdown_listener(tx: Sender<()>) -> io::Result<thread::JoinHandle<()>> {
    #[cfg(unix)]
    {
        unix::install(tx)
    }

    #[cfg(windows)]
    {
        windows::install(tx)
    }
}

#[cfg(unix)]
mod unix {
    use super::*;
    use signal_hook::{consts::signal::*, iterator::Signals};

    pub fn install(tx: Sender<()>) -> io::Result<thread::JoinHandle<()>> {
        let mut signals = Signals::new([SIGINT, SIGTERM, SIGQUIT, SIGHUP])?;

        Ok(thread::spawn(move || {
            for _sig in signals.forever() {
                let _ = tx.try_send(());
            }
        }))
    }
}

#[cfg(windows)]
mod windows {
    use super::*;
    use once_cell::sync::OnceCell;
    use windows_sys::Win32::System::Console::{
        CTRL_BREAK_EVENT,
        CTRL_C_EVENT,
        CTRL_CLOSE_EVENT,
        CTRL_LOGOFF_EVENT,
        CTRL_SHUTDOWN_EVENT,
        SetConsoleCtrlHandler,
    };

    // Lock-free global slot for the sender so the console handler can notify.
    static TX_SLOT: OnceCell<Sender<()>> = OnceCell::new();

    pub fn install(tx: Sender<()>) -> io::Result<thread::JoinHandle<()>> {
        // Ctrl+C / Ctrl+Break via ctrlc
        ctrlc::set_handler({
            let tx = tx.clone();
            move || {
                let _ = tx.try_send(());
            }
        })
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Console window close, user logoff, system shutdown
        enable_console_close_handler(tx)?;

        // Keep a parked thread so caller can Join if desired; handlers are global.
        Ok(std::thread::spawn(|| {
            loop {
                std::thread::park();
            }
        }))
    }

    fn enable_console_close_handler(tx: Sender<()>) -> io::Result<()> {
        // Store the sender once, lock-free.
        let _ = TX_SLOT.set(tx);

        // Minimal, non-blocking handler: just try to send and return quickly.
        #[allow(unsafe_code)]
        unsafe extern "system" fn handler(ctrl: u32) -> i32 {
            match ctrl {
                CTRL_C_EVENT | CTRL_BREAK_EVENT | CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT => {
                    if let Some(tx) = TX_SLOT.get() {
                        let _ = tx.try_send(());
                    }
                    1 // TRUE: handled
                }
                _ => 0, // FALSE: not handled
            }
        }

        #[allow(unsafe_code)]
        unsafe {
            if SetConsoleCtrlHandler(Some(handler), 1) == 0 {
                return Err(io::Error::new(io::ErrorKind::Other, "SetConsoleCtrlHandler failed"));
            }
        }
        Ok(())
    }
}
