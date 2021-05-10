/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 5;
    b: i64 = -5;

    [registers]
    r0: bool = false;
 - i64.in: |
    [main]
    a: i64 = -5;
    b: i64 = 5;

    [registers]
    r0: bool = false;
*/

function main(a: i64, b: i64) -> bool {
    return -a == b;
}
