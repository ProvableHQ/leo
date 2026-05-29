// ANCHOR: file
program vault.aleo {
    mapping balances: address => u64;
    mapping totals:   address => u64;

    view fn get_balance(account: address) -> u64 {
        return balances.get_or_use(account, 0u64);
    }

    // A `final {}` block may call a same-program `view fn`. Unlike a helper
    // `fn` (which is inlined), the view body is run as a separate invocation
    // each time the `final {}` block executes.
    fn cache_total(account: address) -> Final {
        return final {
            let bal: u64 = get_balance(account);
            Mapping::set(totals, account, bal + 100u64);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
