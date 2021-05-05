/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: imports
*/

import test-import.foo as bar;

function main() -> bool {
    return bar() == 1u32;
}
