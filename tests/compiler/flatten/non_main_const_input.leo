/*
namespace: Compile
expectation: Skip
input_file: inputs/dummy.in
*/

function main () -> bool {
    return false;
}

function Fkuty (constant x: bool) -> u8 {
    let y: bool = x;
    y = y;
    return 0u8;
}
