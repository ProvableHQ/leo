function foo() -> field {
    return myGlobal
}

function main() {
    let myGlobal = 42field;
    let err = foo();
}