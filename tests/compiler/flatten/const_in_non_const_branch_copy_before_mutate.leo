/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(b: bool) -> bool {
    let x: u8 = 2u8;
    if b {
        const a: u8 = x;
        for i: u8 in 0u8..a {
            x = x + i;
        }
    } else {
        x = 6u8;
    }
    return x == 1u8 && b;
}