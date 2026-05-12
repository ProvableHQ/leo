// A `final {}` block calls a same-program `query fn` (snarkVM PR #3253).
// The query is not inlined; codegen emits a real `call` instruction
// inside the `finalize` body, which snarkVM accepts because the target
// resolves to a query.
program test.aleo {
    mapping balances: address => u64;
    mapping totals:   address => u64;

    query fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    fn cache_total(addr: address) -> Final {
        return final {
            let bal: u64 = get_balance(addr);
            Mapping::set(totals, addr, bal + 100u64);
        };
    }

    @noupgrade
    constructor() {}
}
