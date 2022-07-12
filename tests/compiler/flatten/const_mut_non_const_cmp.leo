/*
namespace: Compile
expectation: Pass
input_file: inputs/int.in
*/

function main (x: u8) -> bool {
    let y: bool = false;
    y = y != false || x != x;
    return y;
}