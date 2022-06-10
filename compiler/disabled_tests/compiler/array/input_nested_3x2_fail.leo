/*
namespace: Compile
expectation: Fail
input_file: input/input_nested_3x2_fail.in
*/

function main(a: [u8; (3, 2)]) -> bool {
    return a == [[0u8; 2]; 3)]; // This should be written the right way as this test is for the input file.
}
