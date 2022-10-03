/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

transition main(a: i8, b: i8) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
