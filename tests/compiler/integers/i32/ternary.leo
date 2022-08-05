/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: i32, b: i32, c: i32) -> bool {
    let r: i32 = s ? a : b;

    return r == c;
}
