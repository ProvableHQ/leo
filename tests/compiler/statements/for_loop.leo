/*
namespace: Compile
expectation: Pass
input_file: inputs/u32_3.in
*/

function main(x: u32) -> bool {
    let b = 0u32;

    if x == 3 {
        for i in 0..4 {
            b += i;
        }
    }

    let r: u32 = x == 3 ? 6 : 0;

    return r == b;
}
