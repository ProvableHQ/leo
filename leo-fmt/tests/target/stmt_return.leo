program test.aleo {
    transition with_expr(a: u64) -> u64 {
        return a * 2u64;
    }

    transition bare_return() -> () {
        return;
    }

    transition with_tuple(a: u64, b: u64) -> (u64, u64) {
        return (a + b, a * b);
    }
}
