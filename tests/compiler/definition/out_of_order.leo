/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

// @test
// function fake_test() {}

@program
function main(y: bool) -> bool {
    return y == true;
}

// circuit Foo {}
