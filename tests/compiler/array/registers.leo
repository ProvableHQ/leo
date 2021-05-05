/*
namespace: Compile
expectation: Pass
input_file: 
 - input/registers_ones.in
 - input/registers_zeros.in
*/

function main(a: [u8; 3]) -> [u8; 3] {
    return input.registers.r == a ? [3,2,1] : [1,2,3];
}
