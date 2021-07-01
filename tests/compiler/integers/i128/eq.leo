/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 2;
    b: i128 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: i128, b: i128, c: bool) -> bool {
    return (a == b) == c;
}
