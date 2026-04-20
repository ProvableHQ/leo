// Regression test for https://github.com/ProvableHQ/leo/issues/29306
// The compiler should emit a diagnostic error, not panic, when `_dynamic_call`
// is given an undefined type argument and the resulting `Final` is consumed in a final block.
program test.aleo {
    fn main(a: field, b: field, c: field) -> (u32, Final) {
        let (v, f) = _dynamic_call::[(u32, Unknown)](a, b, c);
        return (v, final { f.run(); });
    }
}
