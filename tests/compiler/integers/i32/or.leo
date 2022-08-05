/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: i32, b: i32, c: i32) -> bool {
    return a | b == c;
}
