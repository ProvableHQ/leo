
program test.aleo {
    transition matches(timestamp: i64) {
        assert_eq(timestamp, block.timestamp);
        return;
    }
}
