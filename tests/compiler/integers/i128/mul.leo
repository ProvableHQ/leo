/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 2;
    b: i128 = 5;
    c: i128 = 10;
    
    [registers]
    r0: bool = false;
*/

function main(a: i128, b: i128, c: i128) -> bool {
    return a * b == c;
}
