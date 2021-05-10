/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 1;
    b: i16 = 2;
    c: i16 = 3;
    
    [registers]
    r0: bool = true;
*/

function main(a: i16, b: i16, c: i16) -> bool {
    return a + b == c;
}
