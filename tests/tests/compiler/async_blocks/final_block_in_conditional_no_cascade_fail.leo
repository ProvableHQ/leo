// Regression test: early return after `final_block_in_conditional` error must prevent
// `already_contains_an_async_block` from being set, so that a subsequent valid `final`
// block in the same entry point fn does not spuriously trigger
// `multiple_final_blocks_not_allowed`.
program test.aleo {
    fn foo() -> Final {
        let x: bool = true;
        if x {
            // Invalid: final block inside a conditional.
            let ignored: Final = final { };
        }
        // Valid: this is the only final block at the function level.
        let f: Final = final { };
        return f;
    }
}
