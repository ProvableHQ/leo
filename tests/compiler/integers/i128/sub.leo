/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/sub.in
*/

@program
function main(a: i128, b: i128, c: i128) -> bool {
    return a - b == c;
}
