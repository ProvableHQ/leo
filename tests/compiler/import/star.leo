/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: imports
*/

import test-import.*;

function main() -> bool {
    const a = Point { x: 1u32, y: 0u32 };

    return foo() == 1u32;
}
