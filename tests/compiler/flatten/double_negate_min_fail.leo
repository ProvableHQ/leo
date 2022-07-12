/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

function main() -> bool {
    let x: i8 = --128i8;
    return true;
}