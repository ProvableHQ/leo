/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/sub.in
*/

@program
function main(a: i64, b: i64, c: i64) -> bool {
    return a - b == c;
}
