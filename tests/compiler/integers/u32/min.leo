/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    const a: u32 = 0u32;
    return y == true;
}
