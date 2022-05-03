/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    s: bool = true;
    a: i16 = 10i16;
    b: i16 = 5i16;
    c: i16 = 10i16;

    [registers]
    r0: bool = false;
 - i16_rev.in: |
    [main]
    s: bool = false;
    a: i16 = 10i16;
    b: i16 = 5i16;
    c: i16 = 5i16;

    [registers]
    r0: bool = false;
*/

function main(s: bool, a: i16, b: i16, c: i16) -> bool {
    let r: i16 = s ? a : b;

    return r == c;
}
