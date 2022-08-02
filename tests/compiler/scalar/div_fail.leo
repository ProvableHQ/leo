/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/scalars.in
*/

function main(a: scalar, b: scalar) -> scalar {
    return a / b; // division not supported for scalar types.
}