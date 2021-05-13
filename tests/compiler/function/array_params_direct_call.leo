/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function do_nothing(arr: [u32; 2]) {}

function main(y: bool) -> bool {
    const arr: [u32; 2] = [0; 2];

    do_nothing(arr);
    do_nothing([...arr]);
    do_nothing(arr[0u32..]);

    return y == true;
}
