// Copyright (C) 2019-2021 Aleo Systems Inc.
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

// TODO (protryon): We should merge this with core

use crate::{AsgContext, Program};
use leo_errors::Result;

// TODO (protryon): Make asg deep copy so we can cache resolved core modules
// TODO (protryon): Figure out how to do headers without bogus returns

pub fn resolve_core_module<'a>(context: AsgContext<'a>, module: &str) -> Result<Option<Program<'a>>> {
    match module {
        "unstable.blake2s" => {
            let asg = crate::load_asg(
                context,
                r#"
                circuit Blake2s {
                    function hash(seed: [u8; 32], message: [u8; 32]) -> [u8; 32] {
                        return [0; 32];
                    }
                }
                "#,
                &mut crate::NullImportResolver,
            )?;
            asg.set_core_mapping("blake2s");
            Ok(Some(asg))
        }
        _ => Ok(None),
    }
}
