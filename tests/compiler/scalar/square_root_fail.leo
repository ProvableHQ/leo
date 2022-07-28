/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/scalars.in
*/

function main(a: scalar) -> scalar {
    return a.square_root(); // square root not supported for scalar types.
}