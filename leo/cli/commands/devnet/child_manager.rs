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

use super::*;

//──────────────────────────────────────────────────────────────────────────────
//  Child-process manager
//──────────────────────────────────────────────────────────────────────────────
//
//  Motivation
//  ----------
//  * Every `snarkos` node we start returns a `std::process::Child` handle.
//  * We want **RAII semantics**: whenever the launcher exits—success, error, or panic—all children (and their descendants) must be terminated.
//  * On Unix, we rely on `setsid()` + `killpg()` to nuke the *entire* process-group.
//  * On Windows, we delegate to a Job Object (https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects).
//
//  Design invariants
//  -----------------
//  1.  `ChildManager` owns the list of children **exclusively**.
//      Only the manager is allowed to call `wait`, `try_wait`, or `kill`.
//  2.  All public methods (`push`, `shutdown_all`) are panic-free.
//      Errors only surface via `std::io::Result` or are swallowed during drop.
//  3.  Drop performs a *best-effort* forced shutdown, but never panics—even if system calls fail—so that unwinding during a panic doesn’t abort.
//
//  Typical lifecycle
//  -----------------
//  ```text
//  ┌──────────────────────┐
//  │ spawn children …     │
//  │ manager.push(child)  │
//  └────────┬─────────────┘
//           │                ctrl-C / SIGTERM
//  install_signal_handler ───┘
//           │
//  manager.shutdown_all()   ← explicit or via Drop
//  ```
//
pub struct ChildManager {
    /// All direct `snarkos` children.  
    /// *Invariant*: a `Child` is inserted **exactly once** and removed **never**.
    children: Vec<Child>,
}

impl ChildManager {
    /// Create an empty manager.
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    /// Register a freshly–spawned child so it can be reaped later.
    pub fn push(&mut self, child: Child) {
        self.children.push(child);
    }

    /// Graceful-then-forceful termination sequence:
    ///
    /// 1. Send SIGTERM (Unix) or `.kill()` (Windows) to **every** child.  
    ///    On Unix we target the *process group* (`-PID`) so that any helper processes spawned *by* snarkos die too.
    /// 2. Poll `try_wait` until either
    ///      - **all** children exit, or
    ///      - `timeout` elapses.
    /// 3. Hard-kill survivors with `.kill()` (this is a no-op on already exited processes).
    pub fn shutdown_all(&mut self, timeout: Duration) {
        // ── 1. First “polite” pass ───────────────────────────────────────
        for child in &mut self.children {
            #[cfg(unix)]
            #[allow(unsafe_code)]
            unsafe {
                // SAFETY: we created each child in its own session (setsid),
                // so `-pid` targets precisely that process-group.
                kill(-(child.id() as i32), SIGTERM);
            }
            #[cfg(windows)]
            {
                // `.kill()` sends CTRL-BREAK to console apps or TerminateProcess to GUI apps.
                // The Job Object we attached earlier will do the heavy lifting.
                let _ = child.kill();
            }
        }

        // ── 2. Wait for orderly exit ─────────────────────────────────────
        let start = Instant::now();
        while start.elapsed() < timeout {
            if self.children.iter_mut().all(|c| matches!(c.try_wait(), Ok(Some(_)))) {
                return; // All children exited gracefully.
            }
            thread::sleep(Duration::from_secs(1));
        }

        // ── 3. Escalate to hard kill ─────────────────────────────────────
        for child in &mut self.children {
            let _ = child.kill(); // ignore errors – we’re in teardown
        }
    }
}

impl Drop for ChildManager {
    fn drop(&mut self) {
        // The timeout is hard-coded to 30 seconds.
        // The caller can choose a custom timeout by invoking `shutdown_all` manually before the drop.
        self.shutdown_all(Duration::from_secs(30));
    }
}
