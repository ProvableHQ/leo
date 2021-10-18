/*
namespace: Compile
expectation: Pass
inputs:
 - i16.in: |
    [main]
    a: i16 = 23;
    
    [registers]
    r0: bool = true;
*/

function main(a: i16) -> bool {
    let x_bytes_le = 23i16.to_bytes_le();
    let a_bytes_le = a.to_bytes_le();
    let x_from_le = i16::from_bytes_le(x_bytes_le);

    let x_bytes_be = 23i16.to_bytes_be();
    let a_bytes_be = a.to_bytes_be();
    let x_from_be = i16::from_bytes_be(x_bytes_be);

    return x_bytes_le == a_bytes_le && x_from_le == a
        && x_bytes_be == a_bytes_be && x_from_be == a;
}
