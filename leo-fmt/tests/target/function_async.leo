program test.aleo {
    fn do_something(value: u64) -> Final {
        return final {
            finalize(value);
        };
    }
}

final fn finalize(value: u64) {
    return;
}
