/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: u32, b: u32, c: u32) -> bool {
    let r: u32 = s ? a : b;

    return r == c;
}
