/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main() -> bool {
    let a = [(1u32, 2u32)];
    a[0u32].1 = 3u32;

    return a[0u32].0 == 1u32
        && a[0u32].1 == 3u32;
}
