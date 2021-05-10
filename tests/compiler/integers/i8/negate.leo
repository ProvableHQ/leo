/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 5;
    b: i8 = -5;

    [registers]
    r0: bool = false;
 - i8.in: |
    [main]
    a: i8 = -5;
    b: i8 = 5;

    [registers]
    r0: bool = false;
*/

function main(a: i8, b: i8) -> bool {
    return -a == b;
}
