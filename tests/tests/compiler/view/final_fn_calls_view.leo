// A `final fn` helper that calls a same-program `view fn` gets inlined into the
// `final {}` block that calls the helper. The view call survives inlining and ends
// up as a real `call` instruction inside the resulting Aleo `finalize:` body.
final fn cache(addr: address) {
    let bal: u64 = get_balance(addr);
    Mapping::set(totals, addr, bal + 100u64);
}

program test.aleo {
    mapping balances: address => u64;
    mapping totals:   address => u64;

    view fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    fn cache_total(addr: address) -> Final {
        return final { cache(addr); };
    }

    @noupgrade
    constructor() {}
}
