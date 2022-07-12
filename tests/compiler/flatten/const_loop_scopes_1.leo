/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    for a: u8 in 0u8..2u8 {
        let b: u8 = 1u8;
        let c: u8 = b;
    }
    return true;
}