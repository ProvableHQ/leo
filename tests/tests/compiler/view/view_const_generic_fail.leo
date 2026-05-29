// FAIL: const generics can only be monomorphized at inline call sites. Views are never inlined
// (snarkVM dispatches into the `view` block at run time), so const-generic view fns are rejected.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn read::[N: u32]() -> u32 {
        return N;
    }

    @noupgrade
    constructor() {}
}
