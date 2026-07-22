program test.aleo {
    fn matches(timestamp: i64) {
        assert_eq(timestamp, std::ctx::block_timestamp());
        return;
    }

    @noupgrade
    constructor() {}
}
