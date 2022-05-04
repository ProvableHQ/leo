/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/eq.in
 - inputs/gt.in
*/

function main(a: u16, b: u16, c: bool) -> bool {
    return a > b == c;
}
