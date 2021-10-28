/*
namespace: Compile
expectation: Fail
inputs:
 - bool.in: |
    [main]
    a: bool = true;
    
    [registers]
    r0: bool = true;
*/

function main(a: bool) -> bool {
    let x_bytes_le = true.to_bytes_le();
    let a_bytes_le = a.to_bytes_le();
    let x_from_le = bool::from_bytes_le(x_bytes_le);

    let x_bytes_be = true.to_bytes_be();
    let a_bytes_be = a.to_bytes_be();
    let x_from_be = bool::from_bytes_be(x_bytes_be);

    return x_bytes_le == a_bytes_le && x_from_le == a
        && x_bytes_be == a_bytes_be && x_from_be == a;
}
