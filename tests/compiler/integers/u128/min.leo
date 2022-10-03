/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

transition main(y: bool) -> bool{
    const a: u128 = 0u128;
    return y == true;
}
