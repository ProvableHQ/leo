/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function inf(x: u32) {
    inf(x);
}

function main() {
    inf(0);
}