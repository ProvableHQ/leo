/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
*/

function main(y: bool) -> bool {
    const a: u128 = 340282366920938463463374607431768211455;
    return y == true;
}
