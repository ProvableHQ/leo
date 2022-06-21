/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(x: u8) -> u8 {
    const a: u8 = x;
    const b: u8 = a;
    let c: u8 = x;
    const d: u8 = c;
    return d;
}
