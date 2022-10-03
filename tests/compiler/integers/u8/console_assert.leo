/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

transition main(a: u8, b: u8) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
