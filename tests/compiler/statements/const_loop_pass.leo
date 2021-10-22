/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(y: bool, n: bool) -> bool {
    let res = false;
    for i in 0..range() {
        if i < 5 {
            res = y;
        } else {
            res = n;
        }
    }

    return res == y;
}

const function range() -> u32 {
    return 10;
}
