function main(s: bool, a: field, b: field, c: field) {
    let r = if s ? a : b;

    assert_eq!(r, c);
}