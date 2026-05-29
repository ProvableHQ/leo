// FAIL: queries cannot call other queries (views are leaves; snarkVM rejects `call` inside `view`).
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    view fn inner(x: u64) -> u64 {
        return x + 1u64;
    }

    view fn outer(x: u64) -> u64 {
        return inner(x);
    }

    @noupgrade
    constructor() {}
}
