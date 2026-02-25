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

use leo_errors::CliError;

use snarkvm::prelude::{ConsensusVersion, Network, Program};

/// Threshold percentage for program size warnings.
const PROGRAM_SIZE_WARNING_THRESHOLD: usize = 90;

/// Formats program size as KB and returns a warning message if approaching the limit.
///
/// Both `size` and `max_size` are expected in bytes.
/// Returns `(size_kb, max_kb, warning)` where `warning` is `Some` if size exceeds 90% of max.
pub fn format_program_size(size: usize, max_size: usize) -> (f64, f64, Option<String>) {
    let size_kb = size as f64 / 1024.0;
    let max_kb = max_size as f64 / 1024.0;
    let percentage = (size as f64 / max_size as f64) * 100.0;

    let warning = if size > max_size * PROGRAM_SIZE_WARNING_THRESHOLD / 100 {
        Some(format!("approaching the size limit ({percentage:.1}% of {max_kb:.2} KB)"))
    } else {
        None
    };

    (size_kb, max_kb, warning)
}

/// Default edition for local programs during local operations (run, execute, synthesize).
///
/// Local programs don't have an on-chain edition yet. We default to edition 1 to avoid
/// snarkVM's V8+ check that rejects edition 0 programs without constructors. That check
/// is only relevant for deployed programs, not local development.
pub const LOCAL_PROGRAM_DEFAULT_EDITION: leo_package::Edition = 1;

/// Prints a program's ID and source (local or network edition).
pub fn print_program_source(id: &str, edition: Option<leo_package::Edition>) {
    match (id, edition) {
        ("credits.aleo", _) => println!("  - {id} (already included)"),
        (_, Some(e)) => println!("  - {id} (edition: {e})"),
        (_, None) => println!("  - {id} (local)"),
    }
}

/// Checks if any programs violate edition/constructor requirements.
///
/// Programs at edition 0 without a constructor cannot be executed after ConsensusVersion::V8.
/// This check should be performed before attempting execution to provide a clear error message.
///
/// # Arguments
/// * `programs` - Slice of (program, edition) tuples to check
/// * `consensus_version` - The current consensus version
/// * `action` - Description of the action being attempted (e.g., "deploy", "execute", "upgrade")
///
/// # Returns
/// `Ok(())` if all programs pass the check, or an error with a descriptive message if not.
pub fn check_edition_constructor_requirements<N: Network>(
    programs: &[(Program<N>, leo_package::Edition)],
    consensus_version: ConsensusVersion,
    action: &str,
) -> Result<(), CliError> {
    // Only check for V8+ consensus versions.
    if consensus_version < ConsensusVersion::V8 {
        return Ok(());
    }

    for (program, edition) in programs {
        // Programs at edition 0 without a constructor cannot be executed after V8.
        if *edition == 0 && !program.contains_constructor() {
            let id = program.id();
            // Skip credits.aleo as it's a special case.
            if id.to_string() != "credits.aleo" {
                return Err(CliError::custom(format!(
                    "Cannot {action} with dependency '{id}' (edition 0)\n\n\
                    Programs at edition 0 without a constructor cannot be executed under \
                    consensus version V8 or later (current: V{}).\n\n\
                    The program '{id}' must be upgraded on-chain before it can be used.",
                    consensus_version as u8
                )));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm::prelude::TestnetV0;
    use std::str::FromStr;

    #[test]
    fn test_edition_constructor_error_message() {
        // A program without a constructor at edition 0 should fail under V8+
        let program = Program::<TestnetV0>::from_str(
            "program old_program.aleo;\nfunction main:\n    input r0 as u32.public;\n    output r0 as u32.public;\n",
        )
        .unwrap();

        let result = check_edition_constructor_requirements(&[(program, 0)], ConsensusVersion::V9, "deploy");
        assert!(result.is_err());
    }
}
