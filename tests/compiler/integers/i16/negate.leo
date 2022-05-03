/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

function main(a: i16, b: i16) -> bool {
    return -a == b;
}
