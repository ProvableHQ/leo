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

    return reverse == forward && k;
}