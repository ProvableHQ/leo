/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 1i32;
    b: i32 = 1i32;
*/

function main(a: i32, b: i32) -> i32 {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
