program storage_variable.aleo {
    storage counter: u64;

    fn increment() -> Final {
        return final {
            let current_value: u64 = counter.unwrap_or(0u64);
            counter = current_value + 1u64;
        };
    }

    @noupgrade
    constructor() {}
}
