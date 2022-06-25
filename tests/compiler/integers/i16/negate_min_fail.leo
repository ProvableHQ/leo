/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i16 = -32768i16;
    const b: i16 = -a;
    return (b == -a) == y;
}
