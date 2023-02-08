/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function iteration() -> u32 {
    let a: u32 = 0u32;

    for i: u32 in 0u32..10u32 {
        a = a + 1u32;
    }

    return a;
}

function main(k: bool) -> bool {
    const total: u32 = iteration() + iteration();

    return (total == 20u32) == y;
}
