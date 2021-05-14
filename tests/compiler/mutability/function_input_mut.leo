/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

// Function input are mutable by default.
function main(a: bool) -> bool {
    a = true;

    return a == true;
}
