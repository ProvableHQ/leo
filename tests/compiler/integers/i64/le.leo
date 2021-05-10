/*
namespace: Compile
expectation: Pass
inputs:
 - i64_l.in: |
    [main]
    a: i64 = 1;
    b: i64 = 100;
    c: bool = true;
    
    [registers]
    r0: bool = true;
 - i64_e.in: |
    [main]
    a: i64 = 2;
    b: i64 = 2;
    c: bool = true;
    
    [registers]
    r0: bool = true;
*/

function main(a: i64, b: i64, c: bool) -> bool {
    return a <= b == c;
}
