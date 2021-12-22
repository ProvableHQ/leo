/*
namespace: Import
expectation: Pass
input_file: input/dummy.in
cwd: local_imports
*/

import lib.say_hello;

function main(y: bool) -> bool {
    return y == (say_hello() == "hello");
}
