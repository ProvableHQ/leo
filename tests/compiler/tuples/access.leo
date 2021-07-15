/*
namespace: Compile
expectation: Pass
input_file: inputs/true_true.in
*/

function main(a: (bool, bool)) -> (bool, bool) {
    let b = (false, false, true).0;
    let c = (false, true, false).1;
    let d = (true, false, false).2;

    return (a.0 == b, (a.1 == c) == d);
}
