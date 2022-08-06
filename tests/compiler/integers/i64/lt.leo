/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/lt.in
*/

@program
function main(a: i64, b: i64, c: bool) -> bool{
    return a < b == c;
}
