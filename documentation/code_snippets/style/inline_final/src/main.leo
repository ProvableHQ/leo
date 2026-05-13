program example.aleo {
    mapping accumulator: u8 => u64;

    fn increment_accumulator() -> Final {
        return final {
            let current_count: u64 = accumulator.get_or_use(0u8, 0u64);
            let new_count: u64 = current_count + 1u64;
            accumulator.set(0u8, new_count);
        };
    }

    @noupgrade
    constructor() {}
}
