/*
namespace: Compile
expectation: Pass
input_file:
 - input/false_false.in
 - input/false_true.in
 - input/true_false.in
 - input/true_true.in
*/

function main(a: bool, b: bool) -> bool {
    return a || b;
}