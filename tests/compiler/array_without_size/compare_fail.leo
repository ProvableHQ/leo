/*
namespace: Compile
expectation: Fail
*/

function main() {
    let x: [u8; _] = [1u8,2];
    let z: bool = x == [1u8,2,3]; // array size mismatch
}
