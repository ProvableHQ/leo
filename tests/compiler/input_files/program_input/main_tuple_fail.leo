/*
namespace: Compile
expectation: Fail
input_file: input/main_tuple_fail.in
*/

function main(x: (u8, bool, u8)) {
    console.log("x: {}", x);
}
