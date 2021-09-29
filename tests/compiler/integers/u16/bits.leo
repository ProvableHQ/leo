/*
namespace: Compile
expectation: Pass
inputs:
 - u16.in: |
    [main]
    a: u16 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: u16) -> bool {
    let x_bits = 1u16.to_bits_le();
    let a_bits = a.to_bits_le();

    let x = u16::from_bits_le(x_bits);

    return x_bits == a_bits && x == a;
}
