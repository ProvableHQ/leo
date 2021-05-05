/*
namespace: Compile
expectation: Fail
*/


function foo() -> field {
    return myGlobal;
}

function main() {
    const myGlobal = 42field;
    const err = foo();

    // TODO: update after field comparison is enabled
}
