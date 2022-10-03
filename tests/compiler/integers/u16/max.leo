/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

transition main(y: bool) -> bool {
    const a: u16 = 65535u16;
    return y == true;
}
