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

use leo_asg::*;

mod fail;
mod pass;

fn load_asg(content: &str) -> Result<Program, AsgConvertError> {
    leo_asg::load_asg(content, &mut NullImportResolver)
}

fn load_asg_imports<T: ImportResolver + 'static>(content: &str, imports: &mut T) -> Result<Program, AsgConvertError> {
    leo_asg::load_asg(content, imports)
}

fn mocked_resolver() -> MockedImportResolver {
    let packages = indexmap::IndexMap::new();
    MockedImportResolver { packages }
}
