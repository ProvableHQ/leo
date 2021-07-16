/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 2;
    b: i32 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
*/

function main(a: i32, b: i32, c: bool) -> bool {
    return (a == b) == c;
}
