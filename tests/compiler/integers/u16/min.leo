/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    const a: u16 = 0u16;
    return y == true;
}
