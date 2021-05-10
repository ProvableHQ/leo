/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    s: bool = true;
    a: i128 = 10;
    b: i128 = 5;
    c: i128 = 10;

    [registers]
    r0: bool = false;
 - i128_rev.in: |
    [main]
    s: bool = false;
    a: i128 = 10;
    b: i128 = 5;
    c: i128 = 5;

    [registers]
    r0: bool = false;
*/

function main(s: bool, a: i128, b: i128, c: i128) -> bool {
    let r = s ? a : b;

    return r == c;
}
