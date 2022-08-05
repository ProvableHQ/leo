/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/ne.in
*/

@program
function main(a: i32, b: i32, c: bool) -> bool{
    return (a != b) == c;
}
