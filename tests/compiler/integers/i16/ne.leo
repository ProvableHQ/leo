/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/ne.in
*/

function main(a: i16, b: i16, c: bool) -> bool{
    return (a != b) == c;
}
