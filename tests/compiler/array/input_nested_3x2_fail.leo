/*
namespace: Compile
expectation: Fail
input_file:
 - input/six_zeros.in
 - input/count_to_6.in
*/

function main(a: [u8; (3, 2)]) {
    console.assert(a == [[0u8; 2]; 3)]); // This should be written the right way as this test is for the input file.
}
