/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/string.in
 - inputs/weird.in
*/

function main(s1: [char; 13], s2: [char; 4]) -> bool {
    let hello: [char; 13] = "Hello, World!";
    let part1 = "Good";
    let part2 = " dog!";
    let concat: [char; 9] = [...part1, ...part2];
    return hello == s1 && "nope" != s2 && "es" == s2[1..3] && concat == "Good dog!";
}