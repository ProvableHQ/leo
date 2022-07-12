/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (a: u8) -> u8 {
    for b: u8 in 0u8..2u8 {
        let c: u8 = a;
        c = c;
    }
    return 1u8;
}