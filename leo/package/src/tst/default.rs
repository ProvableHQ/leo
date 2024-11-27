// Copyright (C) 2019-2024 Aleo Systems Inc.
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

//! The default file provided when invoking `leo new` to create a new package.

use crate::tst::directory::TEST_DIRECTORY_NAME;
use leo_errors::{PackageError, Result};

use std::{borrow::Cow, fs::File, io::Write, path::Path};

pub static DEFAULT_TEST_FILENAME: &str = "test.leo";

pub struct DefaultTestFile;

impl DefaultTestFile {
    pub fn write_to(path: &Path) -> Result<()> {
        let mut path = Cow::from(path);
        if path.is_dir() {
            if !path.ends_with(TEST_DIRECTORY_NAME) {
                path.to_mut().push(TEST_DIRECTORY_NAME);
            }
            path.to_mut().push(DEFAULT_TEST_FILENAME);
        }

        let mut file = File::create(&path).map_err(PackageError::io_error_main_file)?;
        Ok(file.write_all(Self::template().as_bytes()).map_err(PackageError::io_error_main_file)?)
    }

    fn template() -> String {
        r#"// A default Leo test file.
// To learn more about testing your program, see the documentation at https://docs.leo-lang.org

@native_test
@interpreted_test
transition test_helloworld() {{
    let result: u32 = helloworld.aleo/main(1u32, 2u32)
    assert_eq!(result, 3u32)
}}
"#
        .to_string()
    }
}
