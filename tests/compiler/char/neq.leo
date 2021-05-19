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

function main(character: char) -> char {
    return character != 'a' ? 'a' : 'Z';
}