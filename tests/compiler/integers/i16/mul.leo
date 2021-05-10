/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 2;
    b: i16 = 5;
    c: i16 = 10;
    
    [registers]
    r0: bool = false;
*/

function main(a: i16, b: i16, c: i16) -> bool {
    return a * b == c;
}
