/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    let reverse: u32 = 0;
    for i in 25..1 {
        reverse += i;
    }

    let reverse_inclusive: u32 = 0;
    for a in 25..=1 {
        reverse_inclusive += a;
    }

    return (reverse + 1 == reverse_inclusive) && k;
}