/*
namespace: Compile
expectation: Fail
input_file: inputs/u32_3.in
*/

function main(x: u32) {
    let x = true ? x: true;
}