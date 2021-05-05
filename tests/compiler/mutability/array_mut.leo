/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main() -> bool {
    let a = [1u32];
    a[0] = 0;

    return a[0] == 0u32;
}
