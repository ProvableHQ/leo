/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
cwd: imports
*/

import money$.foo;

function main() -> bool {
    return false;
}
