/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

function main (b: bool) -> bool {
    for i: u32 in 0u32..10u32 {
            i = 1u32;
    }
    return b;
}