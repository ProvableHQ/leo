/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(constant a: u8) -> bool {
    const b: u8 = 1u8;
    const c: u8 = a;
    const d: u8 = 1u8 + 1u8;
    const h: u8 = d + d + a;
    let e: u8 = b + 1u8;
    let f: u8 = d;
    let g: u8 = 1u8 + f + f + e + a;
    return a + b + c + d + h + e + f + g == 18u8;
}
