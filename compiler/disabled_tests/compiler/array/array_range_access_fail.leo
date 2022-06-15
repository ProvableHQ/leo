/*
namespace: Compile
expectation: Fail
input_file: input/array_range_access_fail.in
*/

function main (
    const x: u32
) {
    const y = [1u8; 3];
    const z: [u8; 2] = y[..1u32][..x];
}