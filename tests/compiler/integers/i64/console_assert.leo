/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 1i64;
    b: i64 = 1i64;
*/

function main(a: i64, b: i64) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
