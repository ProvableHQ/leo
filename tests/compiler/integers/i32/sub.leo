/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 100;
    b: i32 = 50;
    c: i32 = 50;
    
    [registers]
    r0: bool = false;
*/

function main(a: i32, b: i32, c: i32) -> bool {
    return a - b == c;
}
