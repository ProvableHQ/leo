/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: local_imports
*/

import circuits.*;

function main(y: bool) -> bool {
    const a = Point { x: 1u32, y: 0u32 };
    const hello_alias: char5 = "hello";
    const hello = "hello";

    return( (foo() == 1u32) && hello_alias == hello) == y;
}
