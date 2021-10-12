/*
namespace: Compile
expectation: Pass
inputs:
 - bool.in: |
    [main]
    a: bool = true;
    
    [registers]
    r0: bool = true;
*/

function main(a: bool) -> bool {
    let x_bits_le = true.to_bits_le();
    let a_bits_le = a.to_bits_le();
    let x_from_le = bool::from_bits_le(x_bits_le);

    let x_bits_be = true.to_bits_be();
    let a_bits_be = a.to_bits_be();
    let x_from_be = bool::from_bits_be(x_bits_be);

    return x_bits_le == a_bits_le && x_from_le == a
        && x_bits_be == a_bits_be && x_from_be == a;
}
