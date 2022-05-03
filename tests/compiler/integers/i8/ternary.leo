/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    s: bool = true;
    a: i8 = 10i8;
    b: i8 = 5i8;
    c: i8 = 10i8;

    [registers]
    r0: bool = false;
 - i8_rev.in: |
    [main]
    s: bool = false;
    a: i8 = 10i8;
    b: i8 = 5i8;
    c: i8 = 5i8;

    [registers]
    r0: bool = false;
*/

function main(s: bool, a: i8, b: i8, c: i8) -> bool {
    let r: i8 = s ? a : b;

    return r == c;
}
