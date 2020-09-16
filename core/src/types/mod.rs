// Copyright (C) 2019-2020 Aleo Systems Inc.
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

pub mod core_circuit;
pub use self::core_circuit::*;

pub mod core_package;
pub use self::core_package::*;

pub mod core_package_list;
pub use self::core_package_list::*;

pub mod core_symbol_list;
pub use self::core_symbol_list::*;

pub mod value;
pub use self::value::*;

pub static UNSTABLE_CORE_PACKAGE_KEYWORD: &str = "unstable";
