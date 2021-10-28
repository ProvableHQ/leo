/*
namespace: Compile
expectation: Fail
input_file: input/integers.in
*/

function func(x: u32) {
    if x > 100 {
        func(x + 1);
    }
}

function main(a: u32) -> u32 {
    func(a);
    return 1;
}
