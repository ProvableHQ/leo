/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    console.error("hello error");
    return y == true;
}
