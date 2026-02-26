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

use leo_ast::*;
use leo_errors::{Handler, NameValidationError};
use snarkvm::prelude::{Program, TestnetV0};

pub struct NameValidationVisitor<'a> {
    pub handler: &'a mut Handler,
}

impl NameValidationVisitor<'_> {
    pub fn does_not_contain_aleo(&self, name: Identifier, item_type: &str) {
        if name.to_string().contains("aleo") {
            self.handler.emit_err(NameValidationError::illegal_name_content(name, item_type, "aleo", name.span));
        }
    }

    pub fn is_not_keyword(&self, name: Identifier, item_type: &str, whitelist: &[&str]) {
        // Flatten RESTRICTED_KEYWORDS by ignoring ConsensusVersion
        let restricted = Program::<TestnetV0>::RESTRICTED_KEYWORDS.iter().flat_map(|(_, kws)| kws.iter().copied());
        let keywords = Program::<TestnetV0>::KEYWORDS.iter().copied();
        let aleo = std::iter::once("aleo");

        let it = keywords.chain(restricted).chain(aleo).filter(|w| !whitelist.contains(w));

        for word in it {
            if name.to_string() == word {
                self.handler.emit_err(NameValidationError::illegal_name(name, item_type, word, name.span));
                break;
            }
        }
    }
}
