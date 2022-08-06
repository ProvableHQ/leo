/*
namespace: Compile
expectation: Pass
input_files: ../inputs/dummy.in
*/

@program
function main(k: bool) -> bool {
    let b: bool = true;
    b ||= k;

    return b;
}