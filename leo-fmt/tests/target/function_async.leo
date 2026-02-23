program test.aleo {
    async transition do_something(value: u64) -> public Future {
        return finalize(value);
    }

    async function finalize(value: u64) {
        return;
    }
}
