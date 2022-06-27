/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(b: bool) -> bool {
    let x: u8 = 10u8;
    if b {
        x = 5u8;
    } else {
        x = 6u8;
    }
    x = 10u8;
    for i: u8 in 0u8..x {
        let a: bool = false;
    }
    return x == 10u8 && b;
}