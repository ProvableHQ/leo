/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    console.log("{} {}", 1u32, true);
    return y == true;
}
