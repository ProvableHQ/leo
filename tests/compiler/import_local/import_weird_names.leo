/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
cwd: local_imports
*/

import a-9.*;

function main(y: bool) -> bool {
    return y == true;
}
