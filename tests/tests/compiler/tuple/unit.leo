
program test.aleo {
    fn bar(a: u8, b: u8) -> () {
        assert_eq(a, b);
        assert_eq(b, a);
        return;
    }

    fn baz(a: u8, b: u8) -> () {
        assert_eq(a, b);
        assert_eq(b, a);
    }

    fn blar(a: u8, b: u8) {
        assert_eq(a, b);
        assert_eq(b, a);
        return;
    }

    fn blaz(a: u8, b: u8) {
        assert_eq(a, b);
        assert_eq(b, a);
    }
}
