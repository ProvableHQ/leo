/*
namespace: Compile
expectation: Pass
inputs:
 - i32_g.in: |
    [main]
    a: i32 = 4;
    b: i32 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - i32_e.in: |
    [main]
    a: i32 = 4;
    b: i32 = 4;
    c: bool = false;
    
    [registers]
    r0: bool = false;
*/

function main(a: i32, b: i32, c: bool) -> bool {
    return a > b == c;
}
