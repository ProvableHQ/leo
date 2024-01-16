/*
namespace: Compile
expectation: Pass
*/

function main(x: u32) -> bool {
    const COUNT: u32 = 2u32;
    let y: u32 = x;
    
    for i: u32 in 0u32..COUNT {
        y = y - 1u32;
    }

    return y == 1u32;
}
