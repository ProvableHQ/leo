/*
namespace: Compile
expectation: Fail
input_file: inputs/integers.in
*/

// TODO: Change expectation to Pass once inlining is implemented.

@program
function main(a: u32) -> u32 {
    if a == 2u32 {
        return three();
    } else {
        return four();
    } 
}


@inline
function three() -> u32 {
    return 3u32;
}

@inline
function four() -> u32 {
    return 4u32;
}