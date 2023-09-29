/*
namespace: Compile
expectation: Pass
*/

// Function input are mutable by default.
function main(a: bool) -> bool {
    a = true;

    return a == true;
}
