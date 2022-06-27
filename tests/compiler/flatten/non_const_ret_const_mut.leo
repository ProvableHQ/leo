/*
namespace: Compile
expectation: Skip
input_file: inputs/dummy.in
*/

function main (a: bool) -> bool {
    let b: u8 = 1u8;
    b = 2u8;

    if a {
        b = b;
        return b == 2u8;
    } else {
        let c: u8 = b;
        return c == 2u8;
    }
}