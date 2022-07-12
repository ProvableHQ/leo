/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    if true{}
    let b: u8 = 1u8;
    if true {
        let c: u8 = 1u8;
        c = c;
    } else {
        let d: u8 = b;
    }
    return b == 1u8;
}