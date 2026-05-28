program pair_user.aleo {
    // ANCHOR: file
    // Construct a const-generic struct defined in another package.
    fn build() -> pair_lib::Pair::[3u32] {
        return pair_lib::Pair::[3u32] { first: 1u32, second: 2u32 };
    }

    // Call a const-generic helper defined in another package.
    fn triple(x: u32) -> u32 {
        return pair_lib::scale::[3u32](x);
    }
    // ANCHOR_END: file

    @noupgrade
    constructor() {}
}
