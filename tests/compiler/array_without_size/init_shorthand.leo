/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let arr: [u8; _] = [0; 4];
    return y == (arr[0] == 0);
}
