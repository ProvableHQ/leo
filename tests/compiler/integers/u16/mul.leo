/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/mul.in
    
*/

@program
function main(a: u16, b: u16, c: u16) -> bool {
    return a * b == c;
}
