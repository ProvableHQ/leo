/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/ascii.in
 - inputs/escaped_unicode1.in
 - inputs/escaped_unicode2.in
 - inputs/escaped_unicode3.in
 - inputs/escaped_unicode4.in
 - inputs/escaped_unicode5.in
 - inputs/escaped_unicode6.in
 - inputs/escaped.in
 - inputs/hex1.in
 - inputs/hex2.in
 - inputs/unicode1.in
 - inputs/unicode2.in
 - inputs/unicode3.in
 - inputs/unicode4.in
 - inputs/unicode5.in
*/

struct Foo {
    character: char;
}

function main(character: char) -> char {
    let f = Foo { character };

    let character2 = f.character == 'a' ? 'a' : f.character;
    return character2;
}
