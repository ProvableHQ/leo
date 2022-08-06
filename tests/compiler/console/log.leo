/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    console.log("hello world");
    return y == true;
}
