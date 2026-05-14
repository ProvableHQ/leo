// ANCHOR: file
program vault.aleo {
    mapping balances: address => u64;

    // A `view fn` is a read-only entry point. It can read mappings, storage,
    // vectors, `block.height`, and `network.id`, but cannot write any state
    // or call other functions.
    view fn balance_of(account: address) -> u64 {
        return balances.get_or_use(account, 0u64);
    }

    fn deposit(amount: u64) -> Final {
        let caller: address = self.caller;
        return final {
            let current: u64 = Mapping::get_or_use(balances, caller, 0u64);
            Mapping::set(balances, caller, current + amount);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
