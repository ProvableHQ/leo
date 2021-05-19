/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 100;
    b: i64 = 50;
    c: i64 = 50;
    
    [registers]
    r0: bool = false;
*/

function main(a: i64, b: i64, c: i64) -> bool {
    return a - b == c;
}
