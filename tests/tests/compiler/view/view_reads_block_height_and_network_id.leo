// PASS: `std::ctx::block_height()` and `std::ctx::network_id()` are `@_onchain_context`
// wrappers over `FinalizeRead` intrinsics, so views can invoke them.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn read_height() -> u32 {
        return std::ctx::block_height();
    }

    view fn read_network() -> u16 {
        return std::ctx::network_id();
    }

    @noupgrade
    constructor() {}
}
