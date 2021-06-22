/*
namespace: Compile
expectation: Fail
input_file: input/different_types_signed_fail.in
input_file: input/different_types_const_signed_fail.in
input_file: input/main_fail_type.in
*/

function main(const a: i32) {
    let b = a * 2;
}
