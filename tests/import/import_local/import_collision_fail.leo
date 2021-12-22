/*
namespace: Import
expectation: Fail
input_file: input/dummy.in
cwd: local_imports
*/

import collision.collide as howdy;

function main(y: bool) -> bool {
    return y == howdy();
}
