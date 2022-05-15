/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalars.in
*/

function main(a: scalar, b: scalar) -> bool {
    let c: bool = a == b;
    let d: bool = a != b;
    let e: bool = a > b;
    let f: bool = a < b;
    let g: bool = a >= b;
    let h: bool = a <= b;
    return h;
}