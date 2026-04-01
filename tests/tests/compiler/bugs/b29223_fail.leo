// Regression test for https://github.com/ProvableHQ/leo/issues/29223
// The compiler should emit a type error, not panic, when a non-tuple variable
// is accessed with a tuple field index inside a `final` block.
program test.aleo {
    fn foo(p: u32) -> Final {
        let f = final {
            p.0 = 1u32;
        };
        return f;
    }
}
