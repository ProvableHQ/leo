/*
namespace: Compile
expectation: Pass
input_file: inputs/true_true.in
*/

function main(a: (bool, bool)) -> (bool, bool) {
    let b = (a, false);

    return (b.1 || b.0.0, b.1 && b.0.1);
}
