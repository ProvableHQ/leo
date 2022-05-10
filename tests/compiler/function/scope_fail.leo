/*
namespace: Compile
expectation: Fail
*/


function foo() -> field {
    return myGlobal;
}

function main() -> field {
    const myGlobal = 42field;
    const err = foo();

    return err;
}
