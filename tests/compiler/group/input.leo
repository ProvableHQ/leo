/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

@program
function main(a: group, b: group) -> bool {
    console.assert(a == b);
    return a == b;
}