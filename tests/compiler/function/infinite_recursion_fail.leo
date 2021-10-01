/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function inf() {
    inf();
}

function main(y: bool) -> bool {
    inf();
    return y;
}