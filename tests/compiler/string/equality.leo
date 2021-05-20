/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/string.in
*/

function main(s1: [char; 13], s2: [char; 4]) -> bool {
    let hello: [char; 13] = `Hello, World!`;
    return hello == s1 && `nope` != s2;
}