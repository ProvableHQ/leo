/*
namespace: Compile
expectation: Pass
input_file: inputs/true_true.in
*/

function main(a: (bool, bool)) -> (bool, bool) {
    const b = (true, false);

    return (b.0, b.1);
}