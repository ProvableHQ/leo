/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/ascii.in
 - inputs/escaped_unicode.in
 - inputs/escaped.in
 - inputs/hex.in
 - inputs/unicode.in
*/

circuit Foo {
    character: char;
}

function main(character: char) -> char {
    let f = Foo { character };

    let character = f.character == 'a' ? 'a' : 'Z';
    return character;
}