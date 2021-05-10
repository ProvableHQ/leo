/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 1;
    b: i64 = 2;
    c: i64 = 3;
    
    [registers]
    r0: bool = true;
*/

function main(a: i64, b: i64, c: i64) -> bool {
    return a + b == c;
}
