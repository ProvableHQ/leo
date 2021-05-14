/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
*/

function main(y: bool) -> bool {
    const a: u32 = 4294967295;
    return y == true;
}
