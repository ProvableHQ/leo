@no_inline
fn fib(n: u8) -> u8 {
    if n <= 1u8 {
        return n;
    } else {
        return fib(n - 1u8) + fib(n - 2u8);
    }
}

program test.aleo {
    fn foo(n: u8) -> u8 {
        if n <= 1u8 {
            return n;
        } else {
            return foo(n - 1u8) + foo(n - 2u8);
        }
    }
}
