/*
namespace: Compile
expectation: Pass
input_file:
 - ../inputs/dummy.in
*/

transition main(y: bool) -> bool {
    const a: i128 = 0i128;

    return (-a == 0i128) == y;
}
