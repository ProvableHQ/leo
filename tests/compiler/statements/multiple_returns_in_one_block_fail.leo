/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/u32_3.in
*/

function main(x: u32) -> u32 {
    return x;
    let double: u32 = x + x;
    return double;
}
