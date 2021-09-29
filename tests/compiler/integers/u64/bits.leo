/*
namespace: Compile
expectation: Pass
inputs:
 - u64.in: |
    [main]
    a: u64 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: u64) -> bool {
    let x_bits = 1u64.to_bits_le();
    let a_bits = a.to_bits_le();

    let x = u64::from_bits_le(x_bits);

    return x_bits == a_bits && x == a;
}
