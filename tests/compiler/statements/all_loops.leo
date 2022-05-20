/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    let reverse: u32 = 0u32;
    for i: u32 in 9u32..0u32 {
        reverse = reverse + i;
    }

    let forward: u32 = 0u32;
    for x: u32 in 0u32..10u32 {
        forward = forward + x;
    }

    return (reverse == forward) && k;
}