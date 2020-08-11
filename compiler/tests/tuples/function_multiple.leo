function foo() -> (bool, bool) {
    return (true, false)
}

function main() {
    let (a, b) = foo();

    assert_eq!(a, true);
    assert_eq!(b, false);
}