/*
namespace: Compile
expectation: Pass
*/

@test 
function fake_test() {}

function main() {}

// use core import to test import order
import core.unstable.blake2s.Blake2s;

circuit Foo {}