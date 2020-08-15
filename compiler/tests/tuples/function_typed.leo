function foo() -> (bool, bool) {
    return (true, false)
}

function main() {
    let a: (bool, bool) = foo();

    assert_eq!(a.0, true);
    assert_eq!(a.1, false);
}