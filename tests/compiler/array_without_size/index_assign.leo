/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let x: [u8; _] = [0, 1, 2, 3];
    x[2] = 1;
    return y == (x[2] == 1);
}
