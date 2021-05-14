/*
namespace: Compile
expectation: Fail
input_file: input/input_tuple_3x2_fail.in
*/

function main(a: [u8; (3, 2)]) -> bool {
    return a == [0u8; (3, 2)];
}
