function foo() -> field {
    return myGlobal
}

function main() {
    const myGlobal = 42field;
    const err = foo();
}