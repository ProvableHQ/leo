/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    if true {
        let a: u8 = 1u8;
    } else {
        let b: u8 = 1u8;
    }
    let c: u8 = 1u8;
    if true {
        if true {
            c = c;
        }
    }
    return true;
}
