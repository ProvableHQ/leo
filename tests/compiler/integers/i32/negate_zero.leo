/*
namespace: Compile
expectation: Pass
input_file:
 - ../input/dummy.in
*/

function main(y: bool) -> bool {
    const a: i32 = 0i32;

    return (-a == 0i32) == y;
}
