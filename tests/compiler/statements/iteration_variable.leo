/*
namespace: Compile
expectation: Pass
input_file: u32_3.in
*/

function main(x: u32) -> bool {
    const COUNT: u32 = 2;
    let y = x;
    
    for i in 0..COUNT {
        y -= 1;
    }

    return y == 1;
}
