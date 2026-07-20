// Test: an interface must reject a name being used for two different member kinds
// (e.g. a mapping and a storage variable both called `bucket`).

interface Clash {
    mapping bucket: address => u64;
    storage bucket: u64;
}

program test.aleo {
    @noupgrade
    constructor() {}

    fn main(a: u32) -> u32 {
        return a;
    }
}
