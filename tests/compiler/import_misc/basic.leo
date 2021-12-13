/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: imports
*/

import test-import.foo;

function main(y: bool) -> bool {
    return (foo() == 1u32) == true;
}
