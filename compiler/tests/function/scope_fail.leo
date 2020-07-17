function foo() -> field {
    return myGlobal
}

function main() -> field {
    const myGlobal = 42field;
    return foo()
}