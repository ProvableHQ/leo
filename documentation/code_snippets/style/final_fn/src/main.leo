final fn increment_state(acc: u8) {
    let current_count: u64 = accumulator.get_or_use(acc, 0u64);
    let new_count: u64 = current_count + 1u64;
    accumulator.set(acc, new_count);
}

program example.aleo {
    mapping accumulator: u8 => u64;

    fn increment_accumulator() -> Final {
        return final {
            increment_state(0u8);
        };
    }

    @noupgrade
    constructor() {}
}
