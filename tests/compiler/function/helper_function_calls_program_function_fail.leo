/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

function main(a: u32, b: u32, y: bool) -> u32 {
    if y {
       return adder(a, b);
    } else {
        return subber(a, b);
    }
}

@program
function adder(a: u32, b: u32) -> u32 {
    return a + b;
}

@program
function subber(a: u32, b: u32) -> u32 {
    return a - b;
}