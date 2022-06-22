/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (x: u8) -> u8 {
    if true {
        return 0u8;
    } else  {
        const y: i32 = -1i32;
        let z: i32 = y;
        return x;
    }
}