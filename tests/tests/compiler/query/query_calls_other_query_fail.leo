// FAIL: queries cannot call other queries (queries are leaves; snarkVM rejects `call` inside `query`).
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    query fn inner(x: u64) -> u64 {
        return x + 1u64;
    }

    query fn outer(x: u64) -> u64 {
        return inner(x);
    }

    @noupgrade
    constructor() {}
}
