/*
namespace: Compile
expectation: Skip
input_file: inputs/dummy.in
*/

function one() -> u32 {
    return 1u32;
}

function main(y: bool) -> bool {
    let a: u32 = 0u32;

    for i: u32 in 0u32..10u32 {
        a = a + one();
    }

    return (a == 10u32) == y;
}
