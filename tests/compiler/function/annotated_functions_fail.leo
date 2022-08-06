/*
namespace: Compile
expectation: Fail
input_file: inputs/integers.in
*/

@inline
@program
function main(a: u32) -> u32 {
    if a == 2u32 {
        return 3u32;
    } else {
        return 4u32;
    } 
}