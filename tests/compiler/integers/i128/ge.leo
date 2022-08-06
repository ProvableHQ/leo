/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/ge.in
*/

@program
function main(a: i128, b: i128, c: bool) -> bool {
    return a >= b == c;
}
