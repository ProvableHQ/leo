// Regression test for https://github.com/ProvableHQ/leo/issues/29391
// The compiler should emit a diagnostic error, not panic, when a tuple-pattern
// `let` binding has an arity mismatch with its annotated tuple type and any of
// the resulting identifiers (including the surplus one whose type was never
// resolved) is referenced inside a `final {}` block. Before the fix,
// `BlockToFunctionRewriter::rewrite_block` looked up the surplus identifier's
// type and `expect()`ed it, panicking with `must be known by now` after the
// type-checker had already emitted ETYC0372072.
program test.aleo {
    @noupgrade
    constructor() {}

    fn touch() -> Final {
        let (a, b, c): (u32, u32) = (1u32, 2u32);
        return final { let _ = a + b + c; };
    }
}
