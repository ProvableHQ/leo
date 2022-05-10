/*
namespace: Compile
expectation: Pass
input_file: inputs/u32_3.in
*/

function main(x: u32) -> bool {
    let b: u32 = 0u32;

    if x == 3u32 {
        for i: u32 in 0u32..4u32 {
            b = b + i;
        }
    }

    let r: u32 = x == 3u32 ? 6u32 : 0u32;

    return r == b;
}
