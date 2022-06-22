/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(b: bool) -> bool {
    let x: u8 = 0u8;
    if true {
        x = 1u8;
    } else {
        x = 2u8;
    }
    return x == 1u8 && b;
}