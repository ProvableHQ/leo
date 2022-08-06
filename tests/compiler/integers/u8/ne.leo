/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/ne.in
*/

@program
function main(a: u8, b: u8, c: bool) -> bool{
    return (a != b) == c;
}
