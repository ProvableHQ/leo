/*
namespace: Compile
expectation: Fail
inputs:
 - u8.in: |
    [main]
    a: u8 = 23;
    
    [registers]
    r0: bool = true;
*/

function main(a: u8) -> bool {
    let x_bits_le = 23u8.to_bits_le();
    let a_bits_le = a.to_bits_le();
    let x_from_le = u8::from_bits_le(x_bits_le);

    let x_bits_be = 23u8.to_bits_be();
    let a_bits_be = a.to_bits_be();
    let x_from_be = u8::from_bits_be(x_bits_be);

    return x_bits_le == a_bits_le && x_from_le == a
        && x_bits_be == a_bits_be && x_from_be == a;
}
