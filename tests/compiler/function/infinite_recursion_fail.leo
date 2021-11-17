/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function inf(x: u32) {
    inf(x+1);
}

function main() {
    inf(0);
}