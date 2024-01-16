/*
namespace: Compile
expectation: Fail
*/
function main(character: char) -> char {
    console.log("{}", character);

    return character == 'z' ? 'x' : character;
}
