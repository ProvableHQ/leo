/*
namespace: Compile
expectation: Fail
inputs:
 - group.in: |
    [main]
    a: group = 1;
    
    [registers]
    r0: bool = true;
*/

function main(a: group) -> bool {
    let x_bytes_le = 1group.to_bytes_le();
    let a_bytes_le = a.to_bytes_le();
    let x_from_le = group::from_bytes_le(x_bytes_le);

    let x_bytes_be = 1group.to_bytes_be();
    let a_bytes_be = a.to_bytes_be();
    let x_from_be = group::from_bytes_be(x_bytes_be);

    console.log("{}", x_from_le);

    return x_bytes_le == a_bytes_le && x_from_le == a
        && x_bytes_be == a_bytes_be && x_from_be == a;
}
