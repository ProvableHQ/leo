/*
namespace: Import
expectation: Fail
input_file: input/dummy.in
cwd: imports
*/

import a-.foo;

function main() -> bool {
    return false;
}
