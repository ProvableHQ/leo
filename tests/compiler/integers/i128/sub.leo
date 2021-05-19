/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 100;
    b: i128 = 50;
    c: i128 = 50;
    
    [registers]
    r0: bool = false;
*/

function main(a: i128, b: i128, c: i128) -> bool {
    return a - b == c;
}
