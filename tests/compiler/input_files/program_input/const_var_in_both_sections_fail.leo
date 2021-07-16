/*
namespace: Compile
expectation: Fail
input_file: input/var_in_both_sections_fail.in
*/

function main(const a: u32) {
    let b = a * 2;
}
