/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    s: bool = true;
    a: i32 = 10;
    b: i32 = 5;
    c: i32 = 10;

    [registers]
    r0: bool = false;
 - i32_rev.in: |
    [main]
    s: bool = false;
    a: i32 = 10;
    b: i32 = 5;
    c: i32 = 5;

    [registers]
    r0: bool = false;
*/

function main(s: bool, a: i32, b: i32, c: i32) -> bool {
    let r = s ? a : b;

    return r == c;
}
