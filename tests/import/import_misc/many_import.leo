/*
namespace: Import
expectation: Fail
input_file: input/dummy.in
cwd: imports
*/

import test-import.( // local import
    Point,
    foo,
);

import bar.( // IMPOSSIBLE TO IMPORT DIRECTORY
    Bar,
    baz.(Baz, Bazzar),
    bat.bat.Bat,
);

import car.Car; // imports directory import

function main(y: bool) -> bool {}
