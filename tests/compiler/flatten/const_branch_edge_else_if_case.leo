/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    if 1u8 < 2u8 {
        let a: u8 = 1u8;
    } else if true {
        let b: u8 = 1u8;
    } else {
        let c: u8 = 1u8;
    }
    let d: u8 = 1u8;
    {
        let e: u8 = d;
        e = d;
    }
    return true;
}
