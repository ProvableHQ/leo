
program test.aleo {
    fn matches(timestamp: i64) -> Final {
        return finalize_matches(timestamp);
    }
}

onchain fn finalize_matches(timestamp: i64) {
    assert_eq(timestamp, block.timestamp);
}
