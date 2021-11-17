/*
namespace: Compile
expectation: Fail
inputs:
 - char.in: |
    [main]
    a: char = 'a';
    
    [registers]
    r0: bool = true;
*/

function main(a: char) -> bool {
    let x_bytes_le = 'a'.to_bytes_le();
    let a_bytes_le = a.to_bytes_le();
    // let x_from_le = char::from_bytes_le(x_bytes_le);

    let x_bytes_be = 'a'.to_bytes_be();
    let a_bytes_be = a.to_bytes_be();
    // let x_from_be = char::from_bytes_be(x_bytes_be);

    return x_bytes_le == a_bytes_le
        && x_bytes_be == a_bytes_be;
}
