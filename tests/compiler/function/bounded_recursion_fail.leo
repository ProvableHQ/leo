/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function x(const y: u32) {
    if y < 9999u32 {
        x(y+1);
    }
}

function main(y: bool) -> bool {
    x(1u32);
    return y;
}