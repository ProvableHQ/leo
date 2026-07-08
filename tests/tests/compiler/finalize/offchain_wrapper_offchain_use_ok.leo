// PASS: `who` wraps `std::ctx::caller()` (transitively offchain) but is only invoked from an
// entry point body outside a `final {}` block. Off-chain code is exactly where inlining
// `self.caller` is legal, so no error is emitted.
program test.aleo {
    fn main() -> address {
        return who();
    }

    @noupgrade
    constructor() {}
}

export fn who() -> address {
    return std::ctx::caller();
}
