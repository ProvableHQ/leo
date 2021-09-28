/*
namespace: Compile
expectation: Pass
inputs:
 - i32.in: |
    [main]
    a: i32 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: i32) -> bool {
    let x_bits = 1i32.to_bits();
    let a_bits = a.to_bits();

    let x = i32::from_bits(x_bits);

    return x_bits == a_bits && x == a;
}
