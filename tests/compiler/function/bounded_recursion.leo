/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function x(const y: u32) {
    if y < 5u32 {
        x(y+1);
    }
}

function main(y: bool) -> bool {
    x(1u32);
    return y;
}