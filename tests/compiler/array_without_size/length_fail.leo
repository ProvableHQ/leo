/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function main(y: bool) {
    return (10u8.len() == 10u32) == y;
}
