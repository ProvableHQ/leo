/*
namespace: Compile
expectation: Pass
inputs:
 - group.in: |
    [main]
    a: group = 1group;
    
    [registers]
    r0: bool = true;
*/

function main(a: group) -> bool {
    let x_bits_le = 1group.to_bits_le();
    let a_bits_le = a.to_bits_le();
    // let x_from_le = group::from_bits_le(x_bits_le);

    let x_bits_be = 1group.to_bits_be();
    let a_bits_be = a.to_bits_be();
    // let x_from_be = group::from_bits_be(x_bits_be);

    return x_bits_le == a_bits_le
        && x_bits_be == a_bits_be;
}
