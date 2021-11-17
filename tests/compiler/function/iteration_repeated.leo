/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function iteration() -> u32 {
    let a = 0u32;

    for i in 0..10 {
        a += 1;
    }

    return a;
}

function main(y: bool) -> bool {
    let total = iteration() + iteration();

    return (total == 20) == y;
}
