/*
namespace: Compile
expectation: Pass
capture_console: true
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    console.log("hello world");
    return y == true;
}
