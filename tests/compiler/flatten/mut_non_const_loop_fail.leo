/*
namespace: Compile
expectation: Fail
input_file: inputs/int.in
*/

function main(b: u8) -> bool {
    let x: u8 = 10u8;
    x = b;
    for i: u8 in 0u8..x {
        let a: bool = false;
    }
    return x == 1u8 && b == 0u8;
}