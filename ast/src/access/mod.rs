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

pub mod access;
pub use access::*;

pub mod array_access;
pub use array_access::*;

pub mod assignee_access;
pub use assignee_access::*;

pub mod call_access;
pub use call_access::*;

pub mod member_access;
pub use member_access::*;

pub mod static_member_access;
pub use static_member_access::*;

pub mod tuple_access;
pub use tuple_access::*;
