/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    let reverse: u32 = 0;
    for i in 1..0 {
        reverse += i;
    }

    let reverse_inclusive: u32 = 0;
    for a in 1..=0 {
        reverse_inclusive += a;
    }

    return (reverse == 1) && (reverse_inclusive == 1) && k;
}