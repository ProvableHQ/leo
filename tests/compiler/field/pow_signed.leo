/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/fields.in
*/

function main(a: field) -> bool {
    const negOneField: field = -1field;
    return negOneField ** 2i8 == 1field;
}