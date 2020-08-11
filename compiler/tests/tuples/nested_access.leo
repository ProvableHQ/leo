function main() {
    let a = (true, false);
    let b = (true, a);

    assert_eq!(b.0, true);
    assert_eq!(b.1.0, true);
    assert_eq!(b.1.1, false);
}