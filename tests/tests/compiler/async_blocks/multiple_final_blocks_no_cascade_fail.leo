// Regression test: early return after `multiple_final_blocks_not_allowed` error must
// prevent `visit_block` from running on the invalid second `final` block, so that type
// errors inside its body do not cascade.
program test.aleo {
    fn foo() -> Final {
        let f1: Final = final { };
        // Second final block is already invalid (multiple not allowed).
        // Its body contains a type error (bool assigned to u32) that must be suppressed.
        let f2: Final = final {
            let bad: u32 = true;
        };
        return f1;
    }
}
