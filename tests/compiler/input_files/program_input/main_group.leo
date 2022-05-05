/*
namespace: Compile
expectation: Pass
input_file: inputs/main_group.in
*/

function main(a: group, b: group, c: group) -> bool {
    // Change to assert when == is implemented for group.
    console.log("a: {}", a);
    console.log("b: {}", b);
    console.log("c: {}", c);
    return true;
}
