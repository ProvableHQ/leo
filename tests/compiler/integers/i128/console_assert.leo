/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 1;
    b: i128 = 1;
*/

function main(a: i128, b: i128) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
