/*
namespace: Compile
expectation: Pass
inputs:
 - u8_t.in: |
    [main]
    s: bool = true;
    a: u8 = 4u8;
    b: u8 = 2u8;
    c: u8 = 4u8;

    [registers]
    r0: bool = true;
 - u8_f.in: |
    [main]
    s: bool = false;
    b: u8 = 2u8;
    c: u8 = 2u8;
    a: u8 = 4u8;

    [registers]
    r0: bool = true;
*/

function main(s: bool, a: u8, b: u8, c: u8) -> bool {
    let r: u8 = s ? a : b;

    return r == c;
}
