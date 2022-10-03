/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

transition main(y: bool) -> bool {
    const a: u64 = 0u64;
    return y == true;
}
