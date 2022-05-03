/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    s: bool = true;
    a: i64 = 10i64;
    b: i64 = 5i64;
    c: i64 = 10i64;

    [registers]
    r0: bool = false;
 - i64_rev.in: |
    [main]
    s: bool = false;
    a: i64 = 10i64;
    b: i64 = 5i64;
    c: i64 = 5i64;

    [registers]
    r0: bool = false;
*/

function main(s: bool, a: i64, b: i64, c: i64) -> bool {
    let r: i64 = s ? a : b;

    return r == c;
}
