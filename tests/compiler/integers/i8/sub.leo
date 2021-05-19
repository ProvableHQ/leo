/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 100;
    b: i8 = 50;
    c: i8 = 50;
    
    [registers]
    r0: bool = false;
*/

function main(a: i8, b: i8, c: i8) -> bool {
    return a - b == c;
}
