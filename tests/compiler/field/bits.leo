/*
namespace: Compile
expectation: Pass
inputs:
 - field.in: |
    [main]
    a: field = 238;
    
    [registers]
    r0: bool = true;
*/

function main(a: field) -> bool {
    let x_bits_le = 238field.to_bits_le();
    let a_bits_le = a.to_bits_le();
    let x_from_le = field::from_bits_le(x_bits_le);

    let x_bits_be = 238field.to_bits_be();
    let a_bits_be = a.to_bits_be();
    let x_from_be = field::from_bits_be(x_bits_be);

    console.log("{}", x_from_le);

    return x_bits_le == a_bits_le && x_from_le == a
        && x_bits_be == a_bits_be && x_from_be == a;
}
