/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

@program
function main(a: i128, b: i128) -> bool {
    return -a == b;
}
