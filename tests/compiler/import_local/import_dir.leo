/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: local_imports
*/

import nested.hello.say_hello;

function main(y: bool) -> bool {
    return y == (say_hello() == "hello");
}
