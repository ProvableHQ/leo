/*
namespace: Compile
expectation: Pass
input_file: inputs/three.in
*/

function main(a: group, b: group) -> bool {
    // unary
    let e: i8 = a.double();
    let g: i8 = a.neg();

    // binary
    let j: i8 = a.add(b);
    let o: bool = a.eq(b);
    let t: i8 = a.mul(b);
    let w: bool = a.neq(b);

    return a == b;
}