/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

type str = [char; _];

function main(y: bool) -> bool {
    let s = "abc";
    return (first_el(s) == 'a') == y;
}

function first_el(s: str) -> char {
    return s[0];
}
