/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    return (first_el([1,2,3,4]) == 1) == y;
}

function first_el(arr: [u8; _]) -> u8 {
    return arr[0];
}
