/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
*/

function main(y: bool) -> bool{
    const a = -128i64;
    const b = -a;
    return (b == -a) == y;
}
