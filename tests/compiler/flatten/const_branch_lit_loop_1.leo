/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(constant b: bool) -> bool {
    let x: u8 = 10u8;
    if b {
        x = 5u8;
        for i: u8 in 0u8..x {
            let a: bool = false;
        }
    } else {
        x = 6u8;
        for i: u8 in 0u8..x {
            let a: bool = false;
        }
    }

    return x == 6u8;
}