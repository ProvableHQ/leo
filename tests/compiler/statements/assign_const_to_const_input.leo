/*
namespace: Compile
expectation: Pass
input_file: inputs/i8.in
*/

function main(a: i8, constant b: i8) -> i8 {
    const c: i8 = b;
    return c + a;
}