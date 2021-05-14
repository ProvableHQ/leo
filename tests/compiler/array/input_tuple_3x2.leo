/*
namespace: Compile
expectation: Pass
input_file: input/input_tuple_3x2.in
*/

function main(a: [u8; (3, 2)]) -> bool {
    return a == [0u8; (3, 2)];
}
