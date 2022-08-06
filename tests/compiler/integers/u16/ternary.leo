/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: u16, b: u16, c: u16) -> bool {
    let r: u16 = s ? a : b;

    return r == c;
}
