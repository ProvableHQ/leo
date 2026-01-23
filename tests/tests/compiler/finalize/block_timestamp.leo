
program test.aleo {
    fn matches(timestamp: i64) -> Future {
        return finalize_matches(timestamp);
    }
}

onchain fn finalize_matches(timestamp: i64) {
    assert_eq(timestamp, block.timestamp);
}
