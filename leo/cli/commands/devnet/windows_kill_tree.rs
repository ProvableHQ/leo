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

#![allow(unsafe_code)]

//! A cross-platform parity helper for **Windows**.
//! It guarantees that *every* descendant of each `snarkos` node dies when the launcher exits
//! This mirrors the `setsid()` + `killpg()` behaviour on Unix.
//!
//! This is done by putting every spawned child into a single **Job Object** with the `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` flag.
//!
//! ## Usage
//! ```rust,ignore
//! #[cfg(windows)]
//! windows_kill_tree::attach_to_global_job(child.id())?;
//! ```

use anyhow::{Context, Result, anyhow};
use once_cell::sync::OnceCell;
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        JobObjects::{
            AssignProcessToJobObject,
            CreateJobObjectW,
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
            JobObjectExtendedLimitInformation,
            SetInformationJobObject,
        },
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

//──────────────────────────────────────────────────────────────────────────────
//  Job wrapper  (RAII ‖ kill-on-close)
//──────────────────────────────────────────────────────────────────────────────

/// RAII wrapper around a Windows Job Object `HANDLE`.
///
/// The contained `HANDLE` is **non-zero** and owns a job object whose `KILL_ON_JOB_CLOSE` limit flag is set.  
/// It is closed exactly once in `Drop`.
/// The kernel sends `TerminateProcess` to *all* processes in the job the moment the launcher terminates or the variable is dropped.
struct Job(HANDLE);

// Windows handles are just integers.
unsafe impl Send for Job {}
unsafe impl Sync for Job {}

impl Job {
    /// Create a new job with `KILL_ON_JOB_CLOSE`.
    fn create_kill_on_close() -> Result<Self> {
        // SAFETY: passing null security attributes is safe. The job inherits the default DACL (discretionary access control list).
        let handle = unsafe { CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()) } as HANDLE;

        if handle.is_null() {
            return Err(anyhow!("CreateJobObjectW failed"));
        }

        // Configure limit flags.
        let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { core::mem::zeroed() };
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;

        // SAFETY:
        // * `handle` is a valid job object obtained above.
        // * `limits` points to a properly initialised structure.
        // * Size argument matches the struct.
        let ok = unsafe {
            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &mut limits as *mut _ as *mut _,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        };
        if ok == 0 {
            // SAFETY: handle is valid; closing prevents leak before returning Err.
            unsafe { CloseHandle(handle) };
            return Err(anyhow!("SetInformationJobObject failed"));
        }
        Ok(Job(handle))
    }

    /// Attach a PID to this job.
    fn add_process(&self, pid: u32) -> Result<()> {
        // SAFETY:
        // * Desired access `PROCESS_ALL_ACCESS` is permitted for child processes we just spawned.
        // * `FALSE` = do not inherit handle.
        let proc = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, pid) } as HANDLE;

        if proc.is_null() {
            // Child may have exited in the meantime; surface a *non-fatal* error.
            return Err(anyhow!("OpenProcess({pid}) failed")).context("child already exited?");
        }

        // SAFETY:
        // * Both handles reference kernel objects owned by the current process.
        // * They remain valid for the duration of the call.
        let ok = unsafe { AssignProcessToJobObject(self.0, proc) };
        if ok == 0 {
            return Err(anyhow!("AssignProcessToJobObject failed"));
        }
        Ok(())
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        // SAFETY: handle is unique to this RAII wrapper and non-zero by invariant.
        unsafe { CloseHandle(self.0) };
        // Kernel will now terminate every process in the job.
    }
}

//──────────────────────────────────────────────────────────────────────────────
//  Global accessor  (lazy singleton)
//──────────────────────────────────────────────────────────────────────────────

static JOB: OnceCell<Job> = OnceCell::new();
fn global_job() -> Result<&'static Job> {
    JOB.get_or_try_init(Job::create_kill_on_close)
}

//──────────────────────────────────────────────────────────────────────────────
//  Public helper
//──────────────────────────────────────────────────────────────────────────────

/// Attach `pid` to the singleton Job Object.
///
/// Call immediately after each successful `Command::spawn()` on Windows.
/// If the operation fails we log a warning but allow the program to continue.
/// `Child::kill()` still offers best-effort cleanup.
///
/// Returns `Ok(())` if the process was successfully joined **or** if the Job subsystem is unavailable (e.g. ancestor process forbids it).
pub fn attach_to_global_job(pid: u32) -> Result<()> {
    match global_job()?.add_process(pid) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("[windows_kill_tree] WARN: {e:?} – falling back to manual kill");
            Ok(())
        }
    }
}
