function main(a: bool, b: bool, c: u32) {
    let mut d = 0u32;

    if a {
        d += 1;
        if b {
            d += 2;
        }
    }

    assert_eq!(d, c);
}