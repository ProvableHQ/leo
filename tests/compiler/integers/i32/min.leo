/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: i32) -> i32 {
    let b: i32 = -2147483647i32;
    return b - a; // -2147483648i32
}
