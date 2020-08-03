function foo() -> field {
    return myGlobal
}

function main() {
    const myGlobal = 42field;
    let err = foo();
}