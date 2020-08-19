function foo() -> (bool, bool) {
    return (true, false)
}

function main() {
    let (a, b) = foo();

    console.assert(a == true);
    console.assert(b == false);
}