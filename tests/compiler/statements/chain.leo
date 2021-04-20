/*
namespace: Compile
expectation: Pass
input_file:
 - u32_3.in
 - u32_5.in
 - u32_6.in
*/

function main(x: u32) -> bool {
    let c = 0u32;

    if x == 1 {
        c = 1;
    } else if x == 2 {
        c = 2;
    } else {
        c = 3;
    }
    return c == x;
}
