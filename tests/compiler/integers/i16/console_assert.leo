/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 1i16;
    b: i16 = 1i16;
*/

function main(a: i16, b: i16) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
