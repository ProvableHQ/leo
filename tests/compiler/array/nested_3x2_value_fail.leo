/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/


// Multidimensional array syntax in leo
function main(y: bool) -> bool {
    const a: [u32; (3, 2)] = [[0; 3]; 2]; // initializer (incorrectly reversed ordering)
    return false;   
}
