/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 2;
    b: i64 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: i64, b: i64, c: bool) -> bool {
    return (a == b) == c;
}
