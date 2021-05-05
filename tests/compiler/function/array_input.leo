/*
namespace: Compile
expectation: Pass
*/

function foo(a: [u8; 1]) {}

function main() {
    const a: [u8; 1] = [1; 1];
    foo(a);
}
