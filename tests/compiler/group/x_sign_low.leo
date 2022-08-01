/*
namespace: Compile
expectation: Pass
input_file: inputs/scalar_group.in
*/

function main(a: scalar) -> group {
    let b: group = (0, -)group;

    return a * b;
}