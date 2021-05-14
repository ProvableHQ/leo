/*
namespace: Compile
expectation: Pass
input_file:
 - input/six_zeros.in
 - input/count_to_6.in
*/

function main(a: [u8; (3, 2)]) -> bool {
    return a == [0u8; (3, 2)];
}