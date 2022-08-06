/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalars.in
*/

@program
function main(a: scalar) -> bool {
    const s: scalar = 1scalar;
    return s + a == 0scalar;
}