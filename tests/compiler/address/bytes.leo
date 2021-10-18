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
    let x_bytes_le = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8.to_bytes_le();
    let a_bytes_le = a.to_bytes_le();
    let x_from_le = address::from_bytes_le(x_bytes_le);

    let x_bytes_be = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8.to_bytes_be();
    let a_bytes_be = a.to_bytes_be();
    let x_from_be = address::from_bytes_be(x_bytes_be);

    return x_bytes_le == a_bytes_le && x_from_le == a
        && x_bytes_be == a_bytes_be && x_from_be == a;
}
