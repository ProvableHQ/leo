/*
namespace: Compile
expectation: Pass
inputs:
 - i64.in: |
    [main]
    a: i64 = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: i64) -> bool {
    let x_bits = 1i64.to_bits();
    let a_bits = a.to_bits();

    let x = i64::from_bits(x_bits);

    return x_bits == a_bits && x == a;
}
