program test.aleo {
    fn with_expr(a: u64) -> u64 {
        return a * 2u64;
    }

    fn bare_return() -> () {
        return;
    }

    fn with_tuple(a: u64, b: u64) -> (u64, u64) {
        return (a + b, a * b);
    }
}
