// FAIL: a `final fn` helper calls another program's entry-point. After inlining,
// a `call data.aleo/read ...` would land in a finalize body, which snarkVM rejects
// (a finalize `call` must target a view). Type-check rejects up front.
program data.aleo {
    fn read(x: u32) -> u32 {
        return x + 1u32;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import data.aleo;

final fn bad_helper(x: u32) {
    let y: u32 = data.aleo::read(x);
    assert(y == x + 1u32);
}

program caller.aleo {
    fn cache_total(x: u32) -> Final {
        return final { bad_helper(x); };
    }

    @noupgrade
    constructor() {}
}
