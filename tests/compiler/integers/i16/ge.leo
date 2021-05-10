/*
namespace: Compile
expectation: Pass
inputs:
 - i16_e.in: |
    [main]
    a: i16 = 4;
    b: i16 = 4;
    c: bool = true;
    
    [registers]
    r0: bool = true;
 - i16_g.in: |
    [main]
    a: i16 = 5;
    b: i16 = 4;
    c: bool = true;
    
    [registers]
    r0: bool = true;
*/

function main(a: i16, b: i16, c: bool) -> bool {
    return a >= b == c;
}
