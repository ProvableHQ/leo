/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: i16) -> i16 {
    let b: i16 = 32767i16;
    return b - a;
}
