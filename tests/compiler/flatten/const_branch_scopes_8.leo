/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    for x: u32 in 0u32..1u32 {
        if true {}
    }
    if true {
        if false {} else {}
    }
    return true;
}