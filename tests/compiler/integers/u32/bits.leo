/*
namespace: Compile
expectation: Pass
inputs:
 - u32.in: |
    [main]
    a: u32 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: u32) -> bool {
    let x_bits = 1u32.to_bits_le();
    let a_bits = a.to_bits_le();

    let x = u32::from_bits_le(x_bits);

    return x_bits == a_bits && x == a;
}
