/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 2;
    b: i16 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: i16, b: i16, c: bool) -> bool {
    return (a == b) == c;
}
