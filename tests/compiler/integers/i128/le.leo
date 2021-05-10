/*
namespace: Compile
expectation: Pass
inputs:
 - i128_l.in: |
    [main]
    a: i128 = 1;
    b: i128 = 100;
    c: bool = true;
    
    [registers]
    r0: bool = true;
 - i128_e.in: |
    [main]
    a: i128 = 2;
    b: i128 = 2;
    c: bool = true;
    
    [registers]
    r0: bool = true;
*/

function main(a: i128, b: i128, c: bool) -> bool {
    return a <= b == c;
}
