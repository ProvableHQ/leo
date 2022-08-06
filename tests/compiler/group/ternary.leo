/*
namespace: Compile
expectation: Pass
input_file: inputs/point.in
*/

@program
function main(a: group, b: group, c: group) -> bool {
    const r: group = true ? a : b;

    console.assert(r == c);
    return r == c;
}