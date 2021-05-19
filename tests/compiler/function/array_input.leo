/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function foo(a: [u8; 1]) {}

function main(y: bool) -> bool {
    const a: [u8; 1] = [1; 1];
    foo(a);
    return y == true;
}
