/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function one() -> u32 {
    return 1u32;
}

function main(y: bool) -> bool {
    let a = 0u32;

    for i in 0..10 {
        a += one();
    }

    return a == 10u32 == y;
}
