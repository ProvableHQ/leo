function main() {
    let a = (true, false);

    assert_eq!(a.0, true);
    assert_eq!(a.1, false);
}