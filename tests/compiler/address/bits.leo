/*
namespace: Compile
expectation: Pass
inputs:
 - address.in: |
    [main]
    a: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
    
    [registers]
    r0: bool = true;
*/

function main(a: address) -> bool {
    let x_bits_le = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8.to_bits_le();
    let a_bits_le = a.to_bits_le();
    let x_from_le = address::from_bits_le(x_bits_le);

    let x_bits_be = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8.to_bits_be();
    let a_bits_be = a.to_bits_be();
    let x_from_be = address::from_bits_be(x_bits_be);

    return x_bits_le == a_bits_le && x_from_le == a
        && x_bits_be == a_bits_be && x_from_be == a;
}
