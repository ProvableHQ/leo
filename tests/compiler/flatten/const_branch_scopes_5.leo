/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    if true {} else if false {}
    {
        let c: u8 = 1u8;
        c = 1u8;
    }
    return true;
}