/*
namespace: Compile
expectation: Fail
input_file: input/different_types_const_signed_fail.in
*/

function main(const a: i32) {
    let b = a * 2;
}
