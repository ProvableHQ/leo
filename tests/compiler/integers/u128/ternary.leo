/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/tern.in
 - inputs/tern_rev.in
*/

@program
function main(s: bool, a: u128, b: u128, c: u128) -> bool {
    let r: u128 = s ? a : b;

    return r == c;
}
