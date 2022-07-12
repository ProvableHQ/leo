/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i8 = -128i8;
    let z: bool = a == -128i8;

    return z == true;
}
