/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/string.in
 - inputs/weird.in
*/

function main(s1: [char; 13], s2: [char; 4]) -> bool {
    s1[..4] = s2;
    return s1 != "Hello, World!";
}