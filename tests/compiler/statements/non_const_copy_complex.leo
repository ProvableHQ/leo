/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    let x: u8 = 2u8;
    if k {
        const y: u8 = x;
        let b: u8 = 1u8;
        const z: u8 = b;
    }
    return x == 1u8 && k;
}