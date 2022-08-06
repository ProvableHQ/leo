/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/neg.in
 - inputs/neg_rev.in
*/

@program
function main(a: i16, b: i16) -> bool {
    return -a == b;
}
