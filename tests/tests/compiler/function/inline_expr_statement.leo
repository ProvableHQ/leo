
@inline
fn fn_that_returns_a_value() -> u32 {
    return 0u32;
}

@inline
fn another_f_that_returns_a_value(a: u32) -> u32 {
    return a * a;
}

program test.aleo {
    fn some_function() {
        fn_that_returns_a_value();
        let a: u32 = function_that_returns_a_value();
        another_f_that_returns_a_value(a);
        let b: u32 = another_f_that_returns_a_value(a);
    }
}
