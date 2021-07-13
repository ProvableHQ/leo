/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(a: bool) -> bool {
    let x = 2u8;
    let y = x;
    let z = y / 2u8;
    return (z == 1) == a;
}
