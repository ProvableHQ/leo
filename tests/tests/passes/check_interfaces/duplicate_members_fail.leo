// Test: an interface must reject duplicates across every member kind
// (functions, records, mappings, storages) — the reporter of #29560 asked
// that we cover more than just `fn`.

interface Bag {
    fn ping();
    fn ping() -> u32;

    record Token {
        owner: address,
        ..
    }
    record Token {
        owner: address,
        ..
    }

    mapping counts: address => u64;
    mapping counts: address => u64;

    storage total: u64;
    storage total: u64;
}

program test.aleo {
    @noupgrade
    constructor() {}

    fn main(a: u32) -> u32 {
        return a;
    }
}
