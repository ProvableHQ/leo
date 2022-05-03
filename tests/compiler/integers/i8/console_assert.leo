/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 1i8;
    b: i8 = 1i8;
*/

function main(a: i8, b: i8) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}