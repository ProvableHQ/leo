/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/sub.in
*/

@program
function main(a: u32, b: u32, c: u32) -> bool {
    return a - b == c;
}
