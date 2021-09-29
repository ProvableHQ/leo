/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let x: [u8; _] = [1u8,2];
    let z: bool = x == [1u8,2,3]; // array size mismatch
    return y != z;
}
