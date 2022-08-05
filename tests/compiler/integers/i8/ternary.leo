/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: i8, b: i8, c: i8) -> bool {
    let r: i8 = s ? a : b;

    return r == c;
}
