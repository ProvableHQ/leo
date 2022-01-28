/*
namespace: Parse
expectation: Pass
*/

function inf() {
    inf();
}

function main(y: bool) -> bool {
    inf();
    return y;
}