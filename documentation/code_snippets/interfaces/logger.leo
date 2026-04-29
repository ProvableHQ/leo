// ANCHOR: file
interface Logger {
    storage counter: u64;
    storage entries: [u64];
}

program logger.aleo: Logger {
    storage counter: u64;
    storage entries: [u64];

    fn bump(val: u64) -> Final {
        return final {
            counter = counter.unwrap_or(0u64) + 1u64;
            entries.push(val);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
