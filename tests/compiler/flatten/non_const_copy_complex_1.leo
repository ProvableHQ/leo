/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    let x: u8 = 2u8;
    if k {
        const a: u8 = x;
        for i: u8 in 0u8..a {
            x = x + i;
        }
        const c: u8 = x;
        x = 5u8;
    } else {
        x = 6u8;
    }
    return x == 1u8 && k;
}