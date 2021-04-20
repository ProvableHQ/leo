/*
namespace: Compile
expectation: Pass
input_file: u32_3.in
*/

function main(x: u32) -> bool {
    let y = x;
    
    for i in 0..3 {
        y -= 1;
    }

    return y == 0;
}
