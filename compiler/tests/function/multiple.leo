function tuple() -> (bool, bool) {
    return (true, false)
}

function main() {
    let (a, b) = tuple();

    assert_eq!(a, true);
    assert_eq!(b, false);
}