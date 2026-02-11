
program test.aleo {
    fn matches(timestamp: i64) {
        assert_eq(timestamp, block.timestamp);
        return;
    }
}
