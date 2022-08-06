/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: i16, b: i16, c: i16) -> bool {
    let r: i16 = s ? a : b;

    return r == c;
}
