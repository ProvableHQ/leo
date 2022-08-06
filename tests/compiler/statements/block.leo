/*
namespace: Compile
expectation: Pass
input_file: inputs/u32_3.in
*/

@program
function main(x: u32) -> bool {
    let y: u32 = x;
    
    {
        y = y + 5u32;
    }

    return y == 8u32;
}