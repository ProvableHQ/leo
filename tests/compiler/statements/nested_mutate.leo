/*
namespace: Compile
expectation: Pass
input_file:
 - u32_3.in
 - u32_5.in
 - u32_6.in
*/

function main(x: u32) -> bool {
    let d = x;

    if x == 3 || x == 5 {
        d += 1;
        if x == 3 {
            d += 2;
        }
    }

    return d > x;
}