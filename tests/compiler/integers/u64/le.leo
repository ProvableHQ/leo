/*
namespace: Compile
expectation: Pass
inputs:
 - u64_f.in: |
    [main]
    a: u64 = 3;
    b: u64 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u64_e.in: |
    [main]
    a: u64 = 2;
    b: u64 = 2;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u64_l.in: |
    [main]
    a: u64 = 1;
    b: u64 = 2;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u64, b: u64, c: bool) -> bool {
    return a <= b == c;
}
