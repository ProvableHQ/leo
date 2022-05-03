/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: local_imports
*/

import lib.say_hello as howdy;

function main(y: bool) -> bool {
    return y == (howdy() == "hello");
}
