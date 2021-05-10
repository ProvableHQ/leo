/*
namespace: Compile
expectation: Pass
inputs:
 - i128_g.in: |
    [main]
    a: i128 = 4;
    b: i128 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - i128_e.in: |
    [main]
    a: i128 = 4;
    b: i128 = 4;
    c: bool = false;
    
    [registers]
    r0: bool = false;
*/

function main(a: i128, b: i128, c: bool) -> bool {
    return a > b == c;
}
