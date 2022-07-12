/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    for a: u8 in 0u8..1u8 {
        for b: u8 in 0u8..1u8 {}
    }
    return true;
}