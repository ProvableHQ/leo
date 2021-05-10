/*
namespace: Compile
expectation: Pass
inputs:
 - i8_l.in: |
    [main]
    a: i8 = 1;
    b: i8 = 100;
    c: bool = true;
    
    [registers]
    r0: bool = true;
 - i8_e.in: |
    [main]
    a: i8 = 2;
    b: i8 = 2;
    c: bool = true;
    
    [registers]
    r0: bool = true;
*/

function main(a: i8, b: i8, c: bool) -> bool {
    return a <= b == c;
}
