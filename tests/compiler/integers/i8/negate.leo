/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

function main(a: i8, b: i8) -> bool {
    return -a == b;
}
