/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main (b: bool) -> bool {
    let x: bool = false; // false
    if b {
        x = true; // true
    }
    let y: bool = x; // non-const
    return y; // is not propagated
}