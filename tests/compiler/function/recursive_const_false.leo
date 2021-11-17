/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function func(x: u32) {
    if x > 100 {
        func(x + 1);
    }
}

function main(y: bool) -> bool {
    func(1u32);
    return true;
}
