/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(constant a: u8) -> bool {
    const b: i8 = 1i8;
    const c: i8 = -b;
    const d: i8 = !c;
    const e: i8 = !1i8;
    const f: i8 = -1i8;
    const g: bool = -1i8 + f == e;
    return !g == !true;
}
