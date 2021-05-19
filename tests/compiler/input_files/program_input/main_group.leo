/*
namespace: Compile
expectation: Pass
input_file: input/main_group.in
*/

function main(a: group, b: group, c: group) {
    // Change to assert when == is implemented for group.
    console.log("a: {}", a);
    console.log("b: {}", b);
    console.log("c: {}", c);
}
