/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main() -> bool {
    let x: u8 = 0u8;
    if true {
        x = 1u8;
    } else {
        x = 2u8;
    }
    let y: u8 = 0u8;
    if false {
        y = 2u8;
    } else {
        y = 1u8;
    }
    return x == y;
}