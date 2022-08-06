/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/false_false.in
 - inputs/false_true.in
 - inputs/true_false.in
 - inputs/true_true.in
*/

@program
function main(a: bool, b: bool) -> bool {
    return a || b;
}