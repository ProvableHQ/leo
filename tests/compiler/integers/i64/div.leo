/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 4;
    b: i64 = 2;
    c: i64 = 2;
    
    [registers]
    r0: bool = true;
*/

function main(a: i64, b: i64, c: i64) -> bool {
    return a / b == c;
}
