/*
namespace: Compile
expectation: Pass
input_file: inputs/true.in
*/

@program
function main(a: bool) -> bool {
    console.assert(a == true);
    return a == true;
}