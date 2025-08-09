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

use std::{
    process::Child,
    thread,
    time::{Duration, Instant},
};

#[cfg(unix)]
use libc::{SIGKILL, SIGTERM, kill};

/// Manages child processes spawned by `snarkos` commands, ensuring they can be gracefully terminated and reaped.
/// - On Unix, we rely on `setsid()` at spawn + sending signals to the **process group** via `kill(-pid, SIGTERM|SIGKILL)` so helpers die too.
/// - On Windows, we rely on a **Job Object** (created/managed elsewhere) to ensure the tree is contained and hard-killed as a backstop;
///   we still explicitly terminate lingering children during shutdown.
pub struct ChildManager {
    /// All direct `snarkos` children.
    /// *Invariant*: a `Child` is inserted **exactly once** and removed **never**.
    children: Vec<Child>,
    /// Ensures shutdown is **idempotent** even if called multiple times (explicit + Drop).
    shut_down: bool,
}

impl ChildManager {
    /// Create an empty manager.
    pub fn new() -> Self {
        Self { children: Vec::new(), shut_down: false }
    }

    /// Register a freshly–spawned child so it can be reaped later.
    pub fn push(&mut self, child: Child) {
        self.children.push(child);
    }

    /// Graceful-then-forceful termination sequence (idempotent):
    ///
    /// 1) **Polite**: ask everyone to exit.
    ///    - Unix: send `SIGTERM` to the **process group** of each child via `kill(-pid, SIGTERM)`.
    ///    - Windows: no-op by default — rely on app-level handling and Job Objects.
    /// 2) **Wait** up to `timeout` for children to exit (polling `try_wait`).
    /// 3) **Hard kill survivors**:
    ///    - Unix: `kill(-pid, SIGKILL)`.
    ///    - Windows: `Child::kill()` (TerminateProcess). Job Object is a safety net.
    /// 4) **Reap**: call `wait()` on all children to avoid zombies (ignore errors).
    pub fn shutdown_all(&mut self, timeout: Duration) {
        if self.shut_down {
            return;
        }
        self.shut_down = true;

        // ── 1) Polite pass ──────────────────────────────────────────────
        for _child in &mut self.children {
            #[cfg(unix)]
            #[allow(unsafe_code)]
            unsafe {
                // SAFETY: Each child was created with `setsid()` in pre_exec, so `-pid`
                // targets precisely that child’s session / process group.
                let _ = kill(-(_child.id() as i32), SIGTERM);
            }

            #[cfg(windows)]
            {
                // On Windows we skip the "polite" console event unless children were spawned with CREATE_NEW_PROCESS_GROUP (not the default).
                // We rely on:
                //  * application-level graceful handling (Ctrl+C handler), and
                //  * the Job Object (KILL_ON_JOB_CLOSE) for containment.
                // Doing nothing here avoids brittle CTRL_BREAK semantics.
            }
        }

        // ── 2) Wait for orderly exit ────────────────────────────────────
        let start = Instant::now();
        while start.elapsed() < timeout {
            // If all exited, break early; we'll reap below.
            if self.children.iter_mut().all(|c| matches!(c.try_wait(), Ok(Some(_)))) {
                break;
            }
            thread::sleep(Duration::from_millis(150));
        }

        // ── 3) Escalate to hard kill for survivors ──────────────────────
        for child in &mut self.children {
            let still_running = matches!(child.try_wait(), Ok(None));

            #[cfg(unix)]
            if still_running {
                #[allow(unsafe_code)]
                unsafe {
                    let _ = kill(-(child.id() as i32), SIGKILL);
                }
            }

            #[cfg(windows)]
            if still_running {
                // Terminate the process (the Job Object ensures descendants are contained).
                let _ = child.kill();
            }
        }

        // ── 4) Final reap (avoid zombies) ───────────────────────────────
        for child in &mut self.children {
            let _ = child.wait(); // ignore errors during teardown
        }
    }
}

impl Drop for ChildManager {
    fn drop(&mut self) {
        // Best-effort, panic-free shutdown on drop.
        // Callers who need a different timeout should call `shutdown_all` explicitly.
        self.shutdown_all(Duration::from_secs(30));
    }
}
