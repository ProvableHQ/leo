// FAIL: `view fn`s execute off-consensus, so `block.timestamp` is not in scope (snarkVM fills
// it with `None` for view evaluations because there is no transaction-time semantic for an
// off-consensus read). `block.height` and `network.id` are still allowed inside views.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn read_time() -> i64 {
        return block.timestamp;
    }

    @noupgrade
    constructor() {}
}
