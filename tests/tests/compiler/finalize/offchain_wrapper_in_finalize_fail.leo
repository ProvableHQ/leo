// FAIL: `who` wraps `std::ctx::caller()` (an `@offchain` fn) without any annotation. Calling
// it from a `final {}` block would inline `self.caller` into finalize scope. The transitive
// closure over the call graph rejects the callsite; no annotation contagion required.
program test.aleo {
    fn main() -> Final {
        return final { let _ = who(); };
    }

    @noupgrade
    constructor() {}
}

export fn who() -> address {
    return std::ctx::caller();
}
