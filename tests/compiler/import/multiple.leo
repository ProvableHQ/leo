/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: imports
*/

import test-import.(
    Point,
    foo
);

function main() -> bool {
    const a = Point { x: 1u32, y: 0u32 };

    return a.x == 1u32;
}
