/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(y: bool, n: bool) -> bool {
    let res = false;
    for i in 0.."hahaha_good_one".len() {
        if i < 5 {
            res = y;
        } else {
            res = n;
        }
    }

    return res == y;
}
