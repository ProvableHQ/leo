/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(b: bool) -> bool {
    let x: u8 = 1u8;
    x = x;
    if b {
        for i: u8 in 0u8..x {
            let a: bool = false;
        }
    } else {
        for i: u8 in 0u8..x {
            let a: bool = false;
        }
    }
    return x == 1u8;
}