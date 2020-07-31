function main(a: u32) {
    let mut b = 5u32;

    if a == 1 {
        b = 1;
    } else {
        b = 0;
    }

    if a == 1 {
        assert_eq!(b, 1);
    } else {
        assert_eq!(b, 0);
    }
}
