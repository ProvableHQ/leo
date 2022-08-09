/*
namespace: Compile
expectation: Pass
input_file: inputs/integers.in
*/

// TODO: Test framework should print out compiler warnings as well as errors.

@invalid_annotation
function main(a: u32) -> u32 {
    if a == 2u32 {
        return 3u32;
    } else {
        return 4u32;
    } 
}