// ANCHOR: file
program vault.aleo {
    mapping balances: address => u64;
    mapping totals:   address => u64;

    view fn get_balance(account: address) -> u64 {
        return balances.get_or_use(account, 0u64);
    }

    // A `final {}` block may `call` a same-program `view fn`. The view is not
    // inlined: codegen emits a real `call` instruction inside the on-chain
    // body, and snarkVM accepts it because the target resolves to a view.
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
