/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

circuit Bar {
    b2: u32
}

function main(y: bool) -> bool {
    const Bar: u32 = 66u32;

    return y == true;
}
