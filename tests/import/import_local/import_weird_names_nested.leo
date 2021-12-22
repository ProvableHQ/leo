/*
namespace: Import
expectation: Pass
input_file: input/dummy.in
cwd: local_imports
*/

import a-9.nested.c-d.*;

function main(y: bool) -> bool {
    return y == cd();
}
