/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 2;
    b: i64 = 5;
    c: i64 = 10;
    
    [registers]
    r0: bool = false;
*/

function main(a: i64, b: i64, c: i64) -> bool {
    return a * b == c;
}
