/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

// TODO: Compiler output should include warnings.

function never_called_one(a: u8) -> u8 {
    return a + 1u8;
}

@inline
function never_called_two(b: u8) -> u8 {
    return b + 1u8;
}

@program
function main(a: u32, b: u32, y: bool) -> u32 {
    if y {
       return a + b;
    } else {
        return a - b;
    }
}