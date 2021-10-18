/*
namespace: Compile
expectation: Pass
inputs:
 - i128.in: |
    [main]
    a: i128 = 23;
    
    [registers]
    r0: bool = true;
*/

function main(a: i128) -> bool {
    let x_bytes_le = 23i128.to_bytes_le();
    let a_bytes_le = a.to_bytes_le();
    let x_from_le = i128::from_bytes_le(x_bytes_le);

    let x_bytes_be = 23i128.to_bytes_be();
    let a_bytes_be = a.to_bytes_be();
    let x_from_be = i128::from_bytes_be(x_bytes_be);

    return x_bytes_le == a_bytes_le && x_from_le == a
        && x_bytes_be == a_bytes_be && x_from_be == a;
}
