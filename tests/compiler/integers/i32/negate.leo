/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 5;
    b: i32 = -5;

    [registers]
    r0: bool = false;
 - i32.in: |
    [main]
    a: i32 = -5;
    b: i32 = 5;

    [registers]
    r0: bool = false;
*/

function main(a: i32, b: i32) -> bool {
    return -a == b;
}
