/*
namespace: Compile
expectation: Fail
input_file: input/different_types_signed_fail.in
input_file: input/different_types_const_signed_fail.in
input_file: input/var_in_both_sections_fail.in
*/

function main(a: i32) {
    let b = a * 2;
}
