/*
namespace: Compile
expectation: Fail
input_file: inputs/true.in
*/


@program
function main(a: bool) -> bool {
    console.assert_eq(a == 1u8);
    console.assert(1u8);

    return a == true;
}
