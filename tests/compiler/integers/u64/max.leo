/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    const a: u64 = 18446744073709551615u64;
    return y == true;
}
