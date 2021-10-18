/*
namespace: Compile
expectation: Pass
inputs:
 - u16.in: |
    [main]
    a: u16 = 23;
    
    [registers]
    r0: bool = true;
*/

function main(a: u16) -> bool {
    let x_bits_le = 23u16.to_bits_le();
    let a_bits_le = a.to_bits_le();
    let x_from_le = u16::from_bits_le(x_bits_le);

    let x_bits_be = 23u16.to_bits_be();
    let a_bits_be = a.to_bits_be();
    let x_from_be = u16::from_bits_be(x_bits_be);

    return x_bits_le == a_bits_le && x_from_le == a
        && x_bits_be == a_bits_be && x_from_be == a;
}
