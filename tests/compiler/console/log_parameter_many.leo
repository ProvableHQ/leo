/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

@program
function main(y: bool) -> bool {
    console.log("{} {}", 1u32, true);
    return y == true;
}
