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
    let x_bytes_le = 238field.to_bytes_le();
    let a_bytes_le = a.to_bytes_le();
    // let x_from_le = field::from_bytes_le(x_bytes_le);

    let x_bytes_be = 238field.to_bytes_be();
    let a_bytes_be = a.to_bytes_be();
    // let x_from_be = field::from_bytes_be(x_bytes_be);

    return x_bytes_le == a_bytes_le
        && x_bytes_be == a_bytes_be;
}
