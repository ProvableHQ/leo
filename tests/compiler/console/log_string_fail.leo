/*
namespace: Compile
expectation: Fail
input_file: inputs/dummy.in
*/

function main(y: bool) -> bool {
    let hello: string = "hello world";
    console.log(hello);
    return y == true;
}
