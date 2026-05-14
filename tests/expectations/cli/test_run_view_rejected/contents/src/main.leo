// `leo run` against a `view fn` should bail with a clear message — views are read-only and
// `leo run` simulates against an empty in-memory finalize store, so it can't produce a
// meaningful result.
program test_run_view_rejected.aleo {
    mapping balances: address => u64;

    view fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    fn noop(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}
