/*
namespace: Import
expectation: Pass
input_file: input/dummy.in
cwd: imports
*/

import test-import.foo as bar;

function main(y: bool) -> bool {
    return (bar() == 1u32) == y;
}
