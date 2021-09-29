/*
namespace: Compile
expectation: Pass
inputs:
 - u8.in: |
    [main]
    a: u8 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: u8) -> bool {
    let x_bits = 1u8.to_bits_le();
    let a_bits = a.to_bits_le();

    let x = u8::from_bits_le(x_bits);

    return x_bits == a_bits && x == a;
}
