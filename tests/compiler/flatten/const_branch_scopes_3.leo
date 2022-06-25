/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    if true {
        let x: u8 = 1u8;
    }
    {
        let y: u8 = 1u8;
        y = y;
    }
    return true;
}