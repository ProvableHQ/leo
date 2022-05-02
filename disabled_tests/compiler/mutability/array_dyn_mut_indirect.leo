/*
namespace: Compile
expectation: Pass
input_file:
  - input/index1_tuple.in
  - input/index2_tuple.in
*/

function main(i: u32) -> [(u32, u32); 3] {
    let a = [(1u32, 1u32), (2u32, 2u32), (3u32, 3u32)];
    a[i].0 = 0;
    a[i].1 = 1;

    return a;
}
