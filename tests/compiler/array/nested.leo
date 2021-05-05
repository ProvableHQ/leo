/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(n: bool) -> bool {
    const x = [false; (2, 2)];
    const z: bool = x[0][0];

    return n == z;
}
