/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    let a = [1u32, 2u32, 3u32];
    a[0u32..2u32] = [4u32, 5u32];

    return a[0] == 4u32
        && a[1] == 5u32
        && a[2] == 3u32
        && y == true;
}
