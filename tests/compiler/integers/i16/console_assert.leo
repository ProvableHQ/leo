/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

transition main(a: i16, b: i16) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
