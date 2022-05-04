/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

function main(a: i32, b: i32) -> bool {
    return -a == b;
}
