/*
namespace: Compile
expectation: Pass
input_file:
  - input/index1.in
  - input/index2.in
*/

function main(i: u32) -> [u32; 3] {
    let a = [1u32, 2u32, 3u32];
    a[i - 1] += a[i];
    a[i] = 0;

    return a;
}
