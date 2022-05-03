/*
namespace: Compile
expectation: Pass
inputs:
 - u8_g.in: |
    [main]
    a: u8 = 3u8;
    b: u8 = 2u8;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u8_f.in: |
    [main]
    a: u8 = 1u8;
    b: u8 = 2u8;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u8, b: u8, c: bool) -> bool {
    let ret: bool = a > b == c;
    console.assert(ret);
    return ret;
}
