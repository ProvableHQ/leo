/*
namespace: Compile
expectation: Pass
input_file: input/main_field.in
*/

function main(const a: field, const b: field, y: bool) -> bool {
    // Change to assert when == is implemented for field.
    console.log("a: {}", a);
    console.log("b: {}", b);

    return y == true;
}
