// FAIL: `final` and `view` modifiers are mutually exclusive.
program test.aleo {
    fn dummy(x: u32) -> u32 { return x; }

    final view fn nope(x: u64) -> u64 {
        return x;
    }

    @noupgrade
    constructor() {}
}
