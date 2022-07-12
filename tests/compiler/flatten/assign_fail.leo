/*
namespace: Compile
expectation: Fail
input_file: inputs/int.in
*/

function main(x: u8) -> bool {
    const a: u8 = x;
    const b: u8 = a;
    let c: u8 = x;
    const d: u8 = c;
    return d == c;
}
