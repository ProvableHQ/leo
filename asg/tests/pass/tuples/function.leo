function foo() -> (bool, bool) {
    return (true, false);
}

function main() {
    let a = foo();

    console.assert(a.0 == true);
    console.assert(a.1 == false);
}