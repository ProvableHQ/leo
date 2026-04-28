program map.aleo {
    mapping balance: address => u64;

    fn dubble() -> Final {
        let addr: address = self.caller;
        return final {
            let current_value: u64 = balance.get_or_use(addr, 0u64);
            balance.set(addr, current_value + 1u64);

            let next_current_value: u64 = balance.get(addr);
            balance.set(addr, next_current_value + 1u64);
        };
    }

    @noupgrade
    constructor() {}
}
