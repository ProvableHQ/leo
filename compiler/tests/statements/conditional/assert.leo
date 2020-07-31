function main(a: u32) {
    if a == 1 {
        assert_eq!(a, 1);
    } else {
        assert_eq!(a, 0);
    }
}
