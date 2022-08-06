/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalars.in
*/

@program
function main(a: scalar, b: scalar, c: scalar) -> bool {
    return b == 1scalar ? a == 1scalar : c == 2scalar;
}