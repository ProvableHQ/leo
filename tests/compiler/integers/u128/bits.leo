/*
namespace: Compile
expectation: Pass
inputs:
 - u128.in: |
    [main]
    a: u128 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: u128) -> bool {
    let x_bits = 1u128.to_bits_le();
    let a_bits = a.to_bits_le();

    let x = u128::from_bits_le(x_bits);

    return x_bits == a_bits && x == a;
}
