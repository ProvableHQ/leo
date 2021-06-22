/*
namespace: Compile
expectation: Fail
input_file: input/different_types_const_unsigned_fail.in
input_file: input/different_types_unsigned_fail.in
input_file: input/var_in_both_sections_fail.in
*/

function main(a: u32) {
    let b = a * 2;
}
