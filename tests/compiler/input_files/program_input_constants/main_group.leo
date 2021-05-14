/*
namespace: Compile
expectation: Pass
input_file: input/main_group.in
*/

function main(const a: group, const b: group, const c: group, y: bool) -> bool {
    // Change to assert when == is implemented for group.
    console.log("a: {}", a);
    console.log("b: {}", b);
    console.log("c: {}", c);

    return y == true;
}
