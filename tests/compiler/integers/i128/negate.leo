/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 5;
    b: i128 = -5;

    [registers]
    r0: bool = false;
 - i128.in: |
    [main]
    a: i128 = -5;
    b: i128 = 5;

    [registers]
    r0: bool = false;
*/

function main(a: i128, b: i128) -> bool {
    return -a == b;
}
