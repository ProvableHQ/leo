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

struct Foo {}
