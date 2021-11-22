/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/string_out.in
*/

struct Foo {
    s1: [char; 13];
}

function takes_string(s: [char; 13]) -> bool {
    return s == "Hello, World!";
}

function main(s1: [char; 13]) -> [char; 13] {
    let f = Foo { s1 };
    let b = takes_string(s1);
    
    let result = f.s1 == "Hello, World!" ? s1 : "abcdefghjklmn";
    return result;
}