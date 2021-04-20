/*
namespace: Compile
expectation: Pass
input_file: u32_3.in
*/

function main(x: u32) -> bool {
    let y = x;
    
    {
        y += 5u32;
    }

    return y == 8;
}