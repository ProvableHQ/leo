/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    const x: u8 = false? 1u8 : 0u8;
    return x == 0u8;
}
