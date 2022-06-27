/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(constant b: bool) -> bool {
    if b {
        let x: u8 = 1u8;
        x = x;
        for i: u8 in 0u8..x {
            let a: bool = false;
        }
    } else {
        let x: u8 = 1u8;
        x = x;
        for i: u8 in 0u8..x {
            let a: bool = false;
        }
    }
    return !b;
}