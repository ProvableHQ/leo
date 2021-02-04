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

use clap::AppSettings;

pub type NameType = &'static str;

pub type AboutType = &'static str;

pub type DescriptionType = &'static str;

pub type RequiredType = bool;

pub type PossibleValuesType = &'static [&'static str];

pub type IndexType = u64;

pub type ArgumentType = (NameType, DescriptionType, PossibleValuesType, RequiredType, IndexType);

// Format
// "[flag] -f --flag 'Add flag description here'"
pub type FlagType = &'static str;

// Format
// (argument, conflicts, possible_values, requires)
pub type OptionType = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
);

pub type SubCommandType = (
    NameType,
    AboutType,
    &'static [ArgumentType],
    &'static [FlagType],
    &'static [OptionType],
    &'static [AppSettings],
);
