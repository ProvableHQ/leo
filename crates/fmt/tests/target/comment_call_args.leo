fn foo(a: u64, b: u64, c: u64) -> u64 {
    return a + b + c;
}

program test.aleo {
    fn main(public a: u64, public b: u64, public c: u64) -> u64 {
        let short: u64 = foo(
            a, // inline comment
            b,
            c,
        );
        let long: u64 = foo(
            a + b + c + a + b + c + a + b + c + a + b + c + a + b + c + a + b + c,
            /* keep me */
            b + c,
            c + a,
        );
        let trailing: u64 = foo(
            a + b + c + a + b + c + a + b + c + a + b + c + a + b + c + a + b + c,
            b + c,
            c + a,
            // comment after last arg
        );
        return short + long + trailing;
    }
}
