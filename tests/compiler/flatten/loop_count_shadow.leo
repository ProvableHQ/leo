/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (a: bool) -> bool {
    for i: u8 in 0u8..2u8 {
        let b: u8 = 1u8;
    }
    for i: u8 in 0u8..2u8 {
        let b: u8 = 1u8;
    }
    return a;
}