// Regression test for https://github.com/ProvableHQ/leo/issues/29309
// The compiler should emit a diagnostic error, not panic, when a tuple element
// access inside a `final { }` block uses an out-of-bounds index.
program test.aleo {
    fn foo() -> Final {
        let y = (1u32, 2u8, 3u16);
        return final { let _ = y.8; };
    }
}
