function main(s: bool, a: group, b: group, c: group) {
    let r = if s ? a : b;

    assert_eq!(r, c);
}