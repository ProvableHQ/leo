// Regression: a non-async entry-point fn calling another local entry-point fn
// that returns `Final` must be diagnosed as ETYC0372043, not crash type-checking
// with `Finalization not found`. Before the fix, the missing `finalize/$name`
// metadata in `async_function_input_types` reached an unconditional `panic!`
// in `visit_call` after the diagnostic had already been emitted.
program test.aleo {
    fn caller() -> Final {
        return finish();
    }

    fn finish() -> Final {
        return final {};
    }

    @noupgrade
    constructor() {}
}
