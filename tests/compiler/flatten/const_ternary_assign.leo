/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    const a: u8 = 1u8 + (true? 1u8 : 2u8);
    const b: u8 = a;

    const c: u8 = 1u8 - (true? 1u8 : 2u8);
    const d: u8 = c;

    const e: u8 = 1u8 / (true? 1u8 : 2u8);
    const f: u8 = e;

    const g: u8 = 1u8 * (true? 1u8 : 2u8);
    const h: u8 = g;

    const i: u8 = 1u8 ** (true? 1u8 : 2u8);
    const j: u8 = i;

    return a + b + c + d + e + f + g + h + i + j == 10u8;
}