/*
namespace: Compile
expectation: Pass
input_file: inputs/u32_3.in
*/

function main(x: u32) -> bool {
    const COUNT: u32 = 2u32;
    let y: u32 = x;
    
    for i: u32 in 0u32..COUNT {
        y = y - 1u32;
    }

    return y == 1u32;
}
