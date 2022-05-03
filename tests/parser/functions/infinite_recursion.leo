/*
namespace: Parse
expectation: Pass
*/

function inf() -> u8 {
    inf();
}

function main(y: bool) -> bool {
    inf();
    return y;
}