/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    return 1u8 == (true? 1u8 : 2u8);
}