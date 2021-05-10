/*
namespace: Compile
expectation: Pass
inputs:
 - i8_g.in: |
    [main]
    a: i8 = 4;
    b: i8 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - i8_e.in: |
    [main]
    a: i8 = 4;
    b: i8 = 4;
    c: bool = false;
    
    [registers]
    r0: bool = false;
*/

function main(a: i8, b: i8, c: bool) -> bool {
    return a > b == c;
}
