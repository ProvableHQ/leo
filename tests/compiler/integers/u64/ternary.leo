/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: u64, b: u64, c: u64) -> bool {
    let r: u64 = s ? a : b;

    return r == c;
}
