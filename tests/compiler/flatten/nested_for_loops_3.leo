/*
namespace: Compile
expectation: Skip
input_file: inputs/dummy.in
*/

function main (b: bool) -> bool {
    if b {
        let x: u8 = 0u8;
        for i: u8 in 0u8..2u8 {
            let y: u8 = x;
            if true {
                x = 1u8;
                y = x;
            }
            for z: u8 in 0u8..2u8 {
                y = 1u8;
            }
        }
    }

    return true;
}