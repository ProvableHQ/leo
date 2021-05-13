/*
namespace: Compile
expectation: Pass
input_file: 
 - input/input_unequal.in
 - input/input_equal.in
*/

// Conditionally add two u32 integers and log the result to the console.
function main(a: u32, b: u32) -> bool {
    if a == b {
        console.log("{}=={}", a, b); // This line should not fail.
    }
    
    return a == b;
}
