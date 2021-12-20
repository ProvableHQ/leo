/*
namespace: Import
expectation: Fail
input_file: input/dummy.in
cwd: imports
*/

import test-import.*; // local import

import bar.*;         // FORBIDDEN imports directory import
import bar.baz.*;     // FORBIDDEN imports directory import
import bar.bat.bat.*; // FORBIDDEN imports directory import
import car.*;         // FORBIDDEN imports directory import

function main(y: bool) -> bool {}
