/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
cwd: .
*/

import dependency.*;

function main(y: bool) -> bool {
    return y == (Cave::name() == "cave");
}
