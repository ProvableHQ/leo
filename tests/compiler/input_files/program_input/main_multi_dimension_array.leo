/*
namespace: Compile
expectation: Pass
input_file: input/main_multi_dimension_array.in
*/

function main(x: [i16; (2, 2, 3)]) -> bool {
    console.log("x: {}", x);

    const y: [i16; (2, 2, 3)] = [0i16; (2, 2, 3)];
    console.log("y: {}", y);

    return x[0][0][0] == y[0][0][0] && x[1][1][2] == y[1][1][2];
}
