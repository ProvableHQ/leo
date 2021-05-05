/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function one() -> u32 {
    return 1u32;
}

function main() -> bool {
    return one() == 1u32;
}
