/*
namespace: Compile
expectation: Pass
inputs:
 - u64.in: |
    [main]
    a: u64 = 23;
    
    [registers]
    r0: bool = true;
*/

function main(a: u64) -> bool {
    let x_bits_le = 23u64.to_bits_le();
    let a_bits_le = a.to_bits_le();
    let x_from_le = u64::from_bits_le(x_bits_le);

    let x_bits_be = 23u64.to_bits_be();
    let a_bits_be = a.to_bits_be();
    let x_from_be = u64::from_bits_be(x_bits_be);

    console.log("{}", x_from_le);

    return x_bits_le == a_bits_le && x_from_le == a
        && x_bits_be == a_bits_be && x_from_be == a;
}
