/*
namespace: Compile
expectation: Pass
input_file: inputs/int.in
*/

function main (x: bool) -> bool {
    let y: bool = false;
    y = x;
    return y;
}