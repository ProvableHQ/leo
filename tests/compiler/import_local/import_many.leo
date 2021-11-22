/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: local_imports
*/

import structs.(
    Point,
    foo
);

function main(y: bool) -> bool {
    const a = Point { x: 1u32, y: 0u32 };

    return (foo() == 1u32) == y;
}
