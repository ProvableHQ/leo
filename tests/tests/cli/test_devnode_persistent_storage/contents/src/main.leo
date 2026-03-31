// A minimal program used to verify that mapping state persists across devnode restarts.
program persistent_counter.aleo {
    mapping counts: address => u64;

    fn increment() -> Final {
        let caller: address = self.caller;
        return final {
            let current: u64 = counts.get_or_use(caller, 0u64);
            counts.set(caller, current + 1u64);
        };
    }

    @noupgrade
    constructor() {}
}
