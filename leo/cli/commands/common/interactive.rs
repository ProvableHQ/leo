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

use super::*;
use leo_ast::NetworkName;

/// Asks the user to confirm an action, with an optional `--yes` override.
pub fn confirm(prompt: &str, skip_confirmation: bool) -> Result<bool> {
    if skip_confirmation {
        return Ok(true);
    }

    let result = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(false)
        .interact()
        .map_err(|e| CliError::custom(format!("Failed to prompt user: {e}")).into());

    // Print a newline for better formatting.
    println!();

    result
}

/// Logs a warning and asks the user to confirm an action, with an optional `--yes` override.
/// Unlike `confirm`, this function always logs the warning message regardless of `skip_confirmation`.
pub fn warn_and_confirm(warning: &str, skip_confirmation: bool) -> Result<bool> {
    // Always log the warning so users are informed of issues even in non-interactive mode.
    tracing::warn!("{warning}");

    if skip_confirmation {
        return Ok(true);
    }

    let result = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .default(false)
        .interact()
        .map_err(|e| CliError::custom(format!("Failed to prompt user: {e}")).into());

    // Print a newline for better formatting.
    println!();

    result
}

/// Asks the user to confirm a fee.
pub fn confirm_fee<N: Network>(
    fee: &snarkvm::prelude::Fee<N>,
    private_key: &PrivateKey<N>,
    address: &Address<N>,
    endpoint: &str,
    network: NetworkName,
    context: &Context,
    skip: bool,
) -> Result<bool> {
    // Get the fee amount.
    let total_cost = (*fee.amount()? as f64) / 1_000_000.0;
    if fee.is_fee_public() {
        let public_balance = get_public_balance(private_key, endpoint, network, context)? as f64 / 1_000_000.0;
        println!("💰Your current public balance is {public_balance} credits.\n");
        if public_balance < total_cost {
            return Err(PackageError::insufficient_balance(address, public_balance, total_cost).into());
        }
    }
    // Confirm the transaction.
    confirm(&format!("This transaction will cost you {total_cost} credits. Do you want to proceed?"), skip)
}
