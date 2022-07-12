/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (a: bool) -> bool {
    for b: u8 in 0u8..1u8 {
        if true {}
    }
    if a {
        let d: bool = a;
        d = a;
    }
    return a;
}