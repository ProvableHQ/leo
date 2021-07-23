/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(k: bool) -> bool {
    let reverse: u32 = 0;
    for i in 10..0 {
        reverse += i;
    }

    let forward: u32 = 0;
    for x in 0..10 {
        forward += x;
    }

    let reverse_inclusive: u32 = 0;
    for a in 10..=0 {
        reverse_inclusive += a;
    }

    let forward_inclusive: u32 = 0;
    for b in 0..=10 {
        forward_inclusive += b;
    }

    return (reverse == forward) && (reverse_inclusive == forward_inclusive) && k;
}