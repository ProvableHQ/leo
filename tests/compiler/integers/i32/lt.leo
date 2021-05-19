/*
namespace: Compile
expectation: Pass
inputs:
 - i32_l.in: |
    [main]
    a: i32 = 2;
    b: i32 = 4;
    c: bool = true;

    [registers]
    r0: bool = false;
 - i32_e.in: |
    [main]
    a: i32 = 2;
    b: i32 = 2;
    c: bool = false;

    [registers]
    r0: bool = false;
*/

function main(a: i32, b: i32, c: bool) -> bool{
    return a < b == c;
}
