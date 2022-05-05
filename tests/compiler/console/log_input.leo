/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(y: bool) -> bool {
    console.log("a = {}", y);
    return y == true;
}
