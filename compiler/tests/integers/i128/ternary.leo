function main(s: bool, a: i128, b: i128, c: i128) {
    let r = if s ? a : b;

    assert_eq!(r, c);
}