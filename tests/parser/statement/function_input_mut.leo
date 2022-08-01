/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

// Function input are mutable by default.
function main(a: bool) -> bool {
    a = true;

    return a == true;
}
