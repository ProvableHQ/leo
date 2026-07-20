/*
[case]
program = "test.aleo"
function = "read_time"
input = []

[case]
program = "test.aleo"
function = "read_height"
input = []
*/

// `block.timestamp` is readable inside a `view fn`, just like `block.height`. snarkVM populates
// the view's `FinalizeGlobalState` with the block timestamp (present from ConsensusVersion::V12
// onward), so an off-consensus view evaluation returns it. The test harness fabricates a fixed
// timestamp, so `read_time` returns a deterministic value.
program test.aleo {
    fn dummy_transition(x: u32) -> u32 {
        // snarkVM rejects deployments with zero functions, so we keep one minimal transition.
        return x + 1u32;
    }

    view fn read_time() -> i64 {
        return block.timestamp;
    }

    view fn read_height() -> u32 {
        return block.height;
    }

    @noupgrade
    constructor() {}
}
