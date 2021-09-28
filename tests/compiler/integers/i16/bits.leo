/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: i16) -> bool {
    let x_bits = 1i16.to_bits();
    let a_bits = a.to_bits();

    let x = i16::from_bits(x_bits);

    return x_bits == a_bits && x == a;
}
