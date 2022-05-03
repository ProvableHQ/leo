/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    s: bool = true;
    a: i128 = 10i128;
    b: i128 = 5i128;
    c: i128 = 10i128;

    [registers]
    r0: bool = false;
 - i128_rev.in: |
    [main]
    s: bool = false;
    a: i128 = 10i128;
    b: i128 = 5i128;
    c: i128 = 5i128;

    [registers]
    r0: bool = false;
*/

function main(s: bool, a: i128, b: i128, c: i128) -> bool {
    let r: i128 = s ? a : b;

    return r == c;
}
