/*
namespace: Compile
expectation: Pass
input_file: inputs/three.in
*/

@program
function main(a: group, b: group, c: group) -> bool {
    console.assert(a + b == c);

    return a + b == c;
}