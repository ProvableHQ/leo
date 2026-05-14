// `leo execute` against a `view fn` should bail with a clear message — views produce no
// transition and no transaction, so `execute` (which broadcasts a fee-paid transaction) cannot
// target one.
program test_execute_view_rejected.aleo {
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
