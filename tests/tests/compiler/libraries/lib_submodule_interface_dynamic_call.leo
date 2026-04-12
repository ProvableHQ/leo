// Test: dynamic dispatch through an interface defined in a library submodule.

// --- library: ops_lib --- //

// --- Next Module: algorithms.leo --- //

interface Processor {
    fn process(x: u32) -> u32;
}

// --- Next Program --- //

program impl_ops.aleo : ops_lib::algorithms::Processor {
    fn process(x: u32) -> u32 {
        return x + 10u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import impl_ops.aleo;

program caller.aleo {
    fn call_process(target: field, v: u32) -> u32 {
        return ops_lib::algorithms::Processor@(target)::process(v);
    }

    @noupgrade
    constructor() {}
}
