/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 4;
    b: i32 = 2;
    c: i32 = 2;
    
    [registers]
    r0: bool = true;
*/

function main(a: i32, b: i32, c: i32) -> bool {
    return a / b == c;
}
