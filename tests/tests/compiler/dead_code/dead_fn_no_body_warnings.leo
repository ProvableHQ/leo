// A dead (never-called) program-level free function with an unused parameter
// and an unused local. Only the `function is never used` warning must fire:
// the params and locals of a dead function must NOT be reported on top of it
// (matching `rustc`, which suppresses `unused_variables` inside dead code).
//
// Contrast `unused_warnings.leo`, where the analogous `helper` is *live* (called
// from main), so its unused param DOES warn. Liveness is the difference.

program test.aleo {
    fn main() -> u32 {
        return 1u32;
    }

    @noupgrade
    constructor() {}
}

// Never called → reported once as WUNU03714000. Its `unused_param` and
// `unused_local` must produce NO WUNU03714001 warnings.
fn dead_fn(unused_param: u32) -> u32 {
    let unused_local: u32 = 0u32;
    return 5u32;
}
