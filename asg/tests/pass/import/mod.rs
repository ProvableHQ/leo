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

use crate::{load_asg, load_asg_imports, mocked_resolver};

#[test]
fn test_basic() {
    let mut imports = mocked_resolver();
    imports.packages.insert(
        "test-import".to_string(),
        load_asg(include_str!("src/test-import.leo")).unwrap(),
    );
    let program_string = include_str!("basic.leo");
    load_asg_imports(program_string, &mut imports).unwrap();
}

#[test]
fn test_multiple() {
    let mut imports = mocked_resolver();
    imports.packages.insert(
        "test-import".to_string(),
        load_asg(include_str!("src/test-import.leo")).unwrap(),
    );
    let program_string = include_str!("multiple.leo");
    load_asg_imports(program_string, &mut imports).unwrap();
}

#[test]
fn test_star() {
    let mut imports = mocked_resolver();
    imports.packages.insert(
        "test-import".to_string(),
        load_asg(include_str!("src/test-import.leo")).unwrap(),
    );

    let program_string = include_str!("star.leo");
    load_asg_imports(program_string, &mut imports).unwrap();
}

#[test]
fn test_alias() {
    let mut imports = mocked_resolver();
    imports.packages.insert(
        "test-import".to_string(),
        load_asg(include_str!("src/test-import.leo")).unwrap(),
    );

    let program_string = include_str!("alias.leo");
    load_asg_imports(program_string, &mut imports).unwrap();
}

// naming tests
#[test]
fn test_name() {
    let mut imports = mocked_resolver();
    imports.packages.insert(
        "hello-world".to_string(),
        load_asg(include_str!("src/hello-world.leo")).unwrap(),
    );
    imports
        .packages
        .insert("a0-f".to_string(), load_asg(include_str!("src/a0-f.leo")).unwrap());
    imports
        .packages
        .insert("a-9".to_string(), load_asg(include_str!("src/a-9.leo")).unwrap());

    let program_string = include_str!("names.leo");
    load_asg_imports(program_string, &mut imports).unwrap();
}

// more complex tests
#[test]
fn test_many_import() {
    let mut imports = mocked_resolver();
    imports.packages.insert(
        "test-import".to_string(),
        load_asg(include_str!("src/test-import.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar".to_string(),
        load_asg(include_str!("imports/bar/src/lib.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar.baz".to_string(),
        load_asg(include_str!("imports/bar/src/baz.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar.baz".to_string(),
        load_asg(include_str!("imports/bar/src/baz.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar.bat.bat".to_string(),
        load_asg(include_str!("imports/bar/src/bat/bat.leo")).unwrap(),
    );
    imports.packages.insert(
        "car".to_string(),
        load_asg(include_str!("imports/car/src/lib.leo")).unwrap(),
    );

    let program_string = include_str!("many_import.leo");
    load_asg_imports(program_string, &mut imports).unwrap();
}

#[test]
fn test_many_import_star() {
    let mut imports = mocked_resolver();
    imports.packages.insert(
        "test-import".to_string(),
        load_asg(include_str!("src/test-import.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar".to_string(),
        load_asg(include_str!("imports/bar/src/lib.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar.baz".to_string(),
        load_asg(include_str!("imports/bar/src/baz.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar.baz".to_string(),
        load_asg(include_str!("imports/bar/src/baz.leo")).unwrap(),
    );
    imports.packages.insert(
        "bar.bat.bat".to_string(),
        load_asg(include_str!("imports/bar/src/bat/bat.leo")).unwrap(),
    );
    imports.packages.insert(
        "car".to_string(),
        load_asg(include_str!("imports/car/src/lib.leo")).unwrap(),
    );

    let program_string = include_str!("many_import_star.leo");
    load_asg_imports(program_string, &mut imports).unwrap();
}
