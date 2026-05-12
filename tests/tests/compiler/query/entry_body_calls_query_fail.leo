// FAIL: a transition body calls a query directly (not inside `final {}`). Queries
// are only callable from a finalize context (a `final {}` block, a `final fn`, or
// a hoisted `Variant::Finalize`). A top-level `call` to a query in a transition
// section is invalid Aleo.
program test.aleo {
    mapping balances: address => u64;

    query fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    fn cache_total(addr: address) -> u64 {
        // NOT inside final {}. Plain entry-point body calls a query.
        let bal: u64 = get_balance(addr);
        return bal + 100u64;
    }

    @noupgrade
    constructor() {}
}
