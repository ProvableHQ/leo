// `block.timestamp` is readable inside a `view fn`, exactly like `block.height` and `network.id`.
// snarkVM populates the view's `FinalizeGlobalState` with the block timestamp (present from
// ConsensusVersion::V12 onward), so views can read it off-consensus.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn read_time() -> i64 {
        return std::ctx::block_timestamp();
    }

    @noupgrade
    constructor() {}
}
