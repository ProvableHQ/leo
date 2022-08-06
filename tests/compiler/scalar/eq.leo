/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalars.in
*/

@program
function main(a: scalar, b: scalar) -> bool {
    let c: bool = a == b;
    let d: bool = a != b;
    return d;
}