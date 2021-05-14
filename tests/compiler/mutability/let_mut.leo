/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

// Adding the `mut` keyword makes a variable mutable.
function main(y: bool) -> bool {
    let a = 1u32;
    a = 0;

    return a == 0u32 == y;
}
