/*
namespace: Compile
expectation: Fail
*/

function main(character: char) -> char {
    return character != 'a' ? 'a' : 'Z';
}
