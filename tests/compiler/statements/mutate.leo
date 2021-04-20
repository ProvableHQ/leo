/*
namespace: Compile
expectation: Pass
input_file:
 - u32_3.in
 - u32_5.in
*/

function main(x: u32) -> bool {
    let b = 5u32;

    if x == 3 {
        b = 1;
    } else {
        b = 0;
    }

    if x == 3 {
        return (b == 1);
    } else {
        return (b == 0);
    }
}
