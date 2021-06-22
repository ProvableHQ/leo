/*
namespace: Compile
expectation: Fail
input_file: input/different_types_fail.in
input_file: input/main_fail_type.in
*/

function main(a: u32) {
    let b = a * 2;
}
