/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

transition main(a: i32, b: i32) -> bool {
    let ret: bool = a == b;
    console.assert(ret);
    return ret;
}
