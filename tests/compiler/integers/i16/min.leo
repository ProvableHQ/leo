/*
namespace: Compile
expectation: Skip
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i16 = -32768i16;
    return y == true;
}
