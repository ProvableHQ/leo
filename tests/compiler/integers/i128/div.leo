/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 4;
    b: i128 = 2;
    c: i128 = 2;
    
    [registers]
    r0: bool = true;
*/

function main(a: i128, b: i128, c: i128) -> bool {
    return a / b == c;
}
