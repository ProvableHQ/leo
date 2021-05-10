/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 5;
    b: i16 = -5;

    [registers]
    r0: bool = false;
 - i16.in: |
    [main]
    a: i16 = -5;
    b: i16 = 5;

    [registers]
    r0: bool = false;
*/

function main(a: i16, b: i16) -> bool {
    return -a == b;
}
