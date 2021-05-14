/*
namespace: Compile
expectation: Pass
inputs:
 - i16_l.in: |
    [main]
    a: i16 = 2;
    b: i16 = 4;
    c: bool = true;

    [registers]
    r0: bool = false;
 - i16_e.in: |
    [main]
    a: i16 = 2;
    b: i16 = 2;
    c: bool = false;

    [registers]
    r0: bool = false;
*/

function main(a: i16, b: i16, c: bool) -> bool{
    return a < b == c;
}
