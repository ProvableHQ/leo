/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: i128) -> bool {
    let x_bits = 1i128.to_bits();
    let a_bits = a.to_bits();

    let x = i128::from_bits(x_bits);

    return x_bits == a_bits && x == a;
}
