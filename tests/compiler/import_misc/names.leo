/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: imports
*/

import a0-f.foo;
import a-9.bar;
import hello-world.hello;

function main(y: bool) -> bool {
    return y == true;
}
