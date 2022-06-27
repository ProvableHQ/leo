/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i8 = -128i8;
    const b: i8 = -a;

    return (b == -a) == y;
}
