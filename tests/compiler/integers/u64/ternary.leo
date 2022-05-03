/*
namespace: Compile
expectation: Pass
inputs:
 - u64_t.in: |
    [main]
    s: bool = true;
    a: u64 = 4u64;
    b: u64 = 2u64;
    c: u64 = 4u64;

    [registers]
    r0: bool = true;
 - u64_f.in: |
    [main]
    s: bool = false;
    a: u64 = 4u64;
    b: u64 = 2u64;
    c: u64 = 2u64;

    [registers]
    r0: bool = true;
*/

function main(s: bool, a: u64, b: u64, c: u64) -> bool {
    let r: u64 = s ? a : b;

    return r == c;
}
