/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

transition main(y: bool) -> bool {
    const a: u8 = 0u8;
    return y == true;
}
