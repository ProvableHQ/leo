/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function empty() {}

function main(y: bool) -> bool {
    empty();
    return y == true;
}
