// ANCHOR: snippet
// Thin wrapper around an off-chain-only intrinsic. The annotation
// propagates the restriction to every call site, so `who()` is also
// rejected inside `final fn`, `view fn`, or a `final {}` block.
@offchain
fn who() -> address {
    return std::ctx::caller();
}
// ANCHOR_END: snippet

program offchain_demo.aleo {
    // Allowed: an entry `fn` body is off-chain.
    fn whoami() -> address {
        return who();
    }

    @noupgrade
    constructor() {}
}
