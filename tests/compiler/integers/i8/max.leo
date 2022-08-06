/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: i8) -> i8 {
    let b: i8 = 127i8;
    return b - a;
}
