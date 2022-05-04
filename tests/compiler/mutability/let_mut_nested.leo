/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(a: bool) -> bool {
    let x: u8 = 2u8;
    let y: u8 = x;
    let z: u8 = y / 2u8;
    return (z == 1u8) == a;
}
