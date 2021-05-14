/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 100;
    b: i16 = 50;
    c: i16 = 50;
    
    [registers]
    r0: bool = false;
*/

function main(a: i16, b: i16, c: i16) -> bool {
    return a - b == c;
}
