/*
namespace: Compile
expectation: Pass
inputs:
 - i8.in: |
    [main]
    a: i8 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: i8) -> bool {
    let x_bits = 1i8.to_bits_le();
    let a_bits = a.to_bits_le();

    let x = i8::from_bits_le(x_bits);

    return x_bits == a_bits && x == a;
}
