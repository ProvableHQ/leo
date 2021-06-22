/*
namespace: Compile
expectation: Fail
input_file: input/different_types_const_unsigned_fail.in
input_file: input/different_types_unsigned_fail.in
input_file: input/main_fail_type.in
*/

function main(const a: u32) {
    let b = a * 2;
}
