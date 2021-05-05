/*
namespace: Compile
expectation: Pass
input_file: input/main_array.in
*/

function main(const x: [i16; 1]) {
    console.log("{}", x);
    console.assert(x[0] == 0);
}
