/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(a: bool) -> bool {
    let y: u32 = 1u32;
    let x: u32 = 1u32;
    let z: u32 = x + y;

    for i: u32 in 0u32..z {
        y = y + 1u32;
    }

    return (y == 3u32) == a;
}