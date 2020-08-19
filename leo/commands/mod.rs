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

pub mod add;
pub use self::add::*;

pub mod build;
pub use self::build::*;

pub mod clean;
pub use self::clean::*;

pub mod deploy;
pub use self::deploy::*;

pub mod init;
pub use self::init::*;

pub mod lint;
pub use self::lint::*;

pub mod login;
pub use self::login::*;

pub mod new;
pub use self::new::*;

pub mod prove;
pub use self::prove::*;

pub mod publish;
pub use self::publish::*;

pub mod run;
pub use self::run::*;

pub mod setup;
pub use self::setup::*;

pub mod test;
pub use self::test::*;

pub mod remove;
pub use self::remove::*;

pub mod watch;
pub use self::watch::*;
