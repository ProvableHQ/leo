/*
namespace: Compile
expectation: Pass
inputs:
 - u16_g.in: |
    [main]
    a: u16 = 3u16;
    b: u16 = 2u16;
    c: bool = true;

    [registers]
    r0: bool = true;
 - u16_f.in: |
    [main]
    a: u16 = 1u16;
    b: u16 = 2u16;
    c: bool = false;

    [registers]
    r0: bool = true;
*/

function main(a: u16, b: u16, c: bool) -> bool {
    let ret: bool = a > b == c;
    console.assert(ret);
    return ret;
}
