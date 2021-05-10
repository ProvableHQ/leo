/*
namespace: Compile
expectation: Pass
inputs:
 - i32_l.in: |
    [main]
    a: i32 = 1;
    b: i32 = 100;
    c: bool = true;
    
    [registers]
    r0: bool = true;
 - i32_e.in: |
    [main]
    a: i32 = 2;
    b: i32 = 2;
    c: bool = true;
    
    [registers]
    r0: bool = true;
*/

function main(a: i32, b: i32, c: bool) -> bool {
    return a <= b == c;
}
