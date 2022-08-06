/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

@program
function main(a: u16, b: u16) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
