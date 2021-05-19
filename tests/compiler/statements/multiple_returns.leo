/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/u32_3.in
 - inputs/u32_5.in
*/

function main(x: u32) -> bool {
    if x == 3 {
        return true;
    } else {
        return false;
    }
}
