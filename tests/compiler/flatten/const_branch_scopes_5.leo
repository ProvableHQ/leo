/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> u8 {
    if true {} else if false {}
    {
        let c: u8 = 1u8;
        c = 1u8;
    }
    return 1u8;
}