function main(s: bool, a: i32, b: i32, c: i32) {
    let r = if s ? a : b;

    assert_eq!(r, c);
}