/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (a: bool) -> bool {
    for b: u8 in 0u8..1u8 {
        for c: u8 in 0u8..1u8 {}
        if a {}
    }
    return true;
}