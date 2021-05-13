/*
namespace: Compile
expectation: Pass
input_file: input/main_array.in
*/

function main(const x: [i16; 1], y: bool) -> bool {
    console.log("{}", x);
    return x[0] == 0 == y;
}
