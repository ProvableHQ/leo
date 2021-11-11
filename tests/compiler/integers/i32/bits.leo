/*
namespace: Compile
expectation: Fail
inputs:
 - i32.in: |
    [main]
    a: i32 = 23;
    
    [registers]
    r0: bool = true;
*/

function main(a: i32) -> bool {
    let x_bits_le = 23i32.to_bits_le();
    let a_bits_le = a.to_bits_le();
    let x_from_le = i32::from_bits_le(x_bits_le);

    let x_bits_be = 23i32.to_bits_be();
    let a_bits_be = a.to_bits_be();
    let x_from_be = i32::from_bits_be(x_bits_be);

    return x_bits_le == a_bits_le && x_from_le == a
        && x_bits_be == a_bits_be && x_from_be == a;
}
