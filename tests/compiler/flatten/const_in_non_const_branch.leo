/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(b: bool) -> bool {
    let x: u8 = 10u8;
    if b {
        const a: u8 = 24u8;
        x = 5u8;
    } else {
        x = 6u8;
    }
    return x == 1u8 && b;
}