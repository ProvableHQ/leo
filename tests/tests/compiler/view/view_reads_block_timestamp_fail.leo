// FAIL: `view fn`s execute off-consensus, so `std::ctx::block_timestamp()` is not available.
// `std::ctx::block_height()` and `std::ctx::network_id()` are still allowed inside views.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn read_time() -> i64 {
        return std::ctx::block_timestamp();
    }

    @noupgrade
    constructor() {}
}
