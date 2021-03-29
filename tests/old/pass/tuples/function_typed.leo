function foo() -> (bool, bool) {
    return (true, false)
}

function main() {
    const a: (bool, bool) = foo();

    console.assert(a.0 == true);
    console.assert(a.1 == false);
}