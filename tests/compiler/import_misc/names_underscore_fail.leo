/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
cwd: imports
*/

import hello_world.foo;

function main() -> bool {
    return true;
}
