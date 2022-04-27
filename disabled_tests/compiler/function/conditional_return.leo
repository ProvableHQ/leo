/*
namespace: Compile
expectation: Pass
input_file: input/integers.in
*/

function main(a: u32) -> u32 {
    if a == 2u32 {
        return 3u32;
    } else {
        return 4u32;
    } 
}
