/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    s: bool = true;
    a: i16 = 10;
    b: i16 = 5;
    c: i16 = 10;

    [registers]
    r0: bool = false;
 - i16_rev.in: |
    [main]
    s: bool = false;
    a: i16 = 10;
    b: i16 = 5;
    c: i16 = 5;

    [registers]
    r0: bool = false;
*/

function main(s: bool, a: i16, b: i16, c: i16) -> bool {
    let r = s ? a : b;

    return r == c;
}
