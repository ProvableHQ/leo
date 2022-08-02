/*
namespace: Compile
expectation: Pass
input_file: inputs/scalar_group.in
*/

function main(a: scalar) -> group {
    let element: group = 0group;

    return element * a;
}