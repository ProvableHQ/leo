// FAIL: `final` and `query` modifiers are mutually exclusive.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    final query fn nope(x: u64) -> u64 {
        return x;
    }

    @noupgrade
    constructor() {}
}
