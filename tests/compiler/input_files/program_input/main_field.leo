/*
namespace: Compile
expectation: Pass
input_file: input/main_field.in
*/

function main(a: field, b: field, y: bool) -> bool {
    return y == true && a == b;
}
