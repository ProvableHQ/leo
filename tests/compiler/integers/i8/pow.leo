/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 2;
    b: i8 = 2;
    c: i8 = 4;
    
    [registers]
    r0: bool = true;
*/

function main(a: i8, b: i8, c: i8) -> bool {
    return a ** b == c;
}
