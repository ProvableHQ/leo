/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

@test 
function fake_test() {}

function main(y: bool) -> bool {
    return y == true;
}

// use core import to test import order
import core.unstable.blake2s.Blake2s;

circuit Foo {}
