/*
namespace: Compile
expectation: Pass
input_file: input/main_field.in
*/

function main(const a: field, const b: field, y: bool) -> bool {
    return y == true && a == b;
}
