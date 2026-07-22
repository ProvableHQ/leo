// FAIL: direct use of an off-chain-only intrinsic inside a `view fn`. The old error told the
// user to move the call into a `final fn` or `final { … }` block, which is also invalid there
// — this now emits a view-specific message that names the actual constraint.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn who() -> address {
        return _self_caller();
    }

    view fn signed_by() -> address {
        return _self_signer();
    }

    @noupgrade
    constructor() {}
}
