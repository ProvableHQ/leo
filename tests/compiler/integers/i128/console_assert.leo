/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

@program
function main(a: i128, b: i128) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
