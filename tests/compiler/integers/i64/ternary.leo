/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: i64, b: i64, c: i64) -> bool {
    let r: i64 = s ? a : b;

    return r == c;
}
