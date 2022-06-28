/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (b: bool) -> bool {
    let x: u8 = 0u8;
    for i: u8 in 0u8..2u8 {
        let y: u8 = x;
        if b {
            x = y;
            y = y;
        }
    }
    return x == 0u8;
}